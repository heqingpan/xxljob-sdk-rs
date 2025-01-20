# xxljob-sdk-rs


## 介绍

rust实现的xxl-job sdk客户端。



## 使用方式

#### 1. 首先在项目加入引用

```
[dependencies]
xxljob-sdk-rs = "0.1.1"
```

#### 2. 创建客户端

使用XxlClientBuilder构建创建客户端。

```rust
let client = XxlClientBuilder::new("http://127.0.0.1:8080/xxl-job-admin".to_string())
        .set_access_token("default_token".to_string())
        .set_log_path("xxl-rs-logs".to_string())
        .set_app_name("xxl-job-executor-sample".to_string())
        .build()?;
```

创建客户端后会同时设置到全局变量中，后续可以通过`get_last_xxl_client()`获取的最近的客户端。

#### 3. 实现任务处理器

xxljob-sdk-rs同时支持同步、异步任务处理器。

+ 异步任务处理器，只使用一个异步线程运行所有任务；不要在内容写同步堵塞线程逻辑。常规任务、io密集型任务推荐使用。
+ 同步任务处理器， 每个任务起一个线程运行；CPU密集型任务推荐使用。



```rust
pub struct DemoJobHandler;

#[async_trait]
impl AsyncJobHandler for DemoJobHandler {
    async fn process(&self, context: JobContext) -> anyhow::Result<JobContext> {
        log::info!(
            "async|DemoJobHandler job info; job_id:{}, log_id:{}, job_param:{:?}",
            &context.job_id,
            &context.log_id,
            &context.job_param
        );
        for i in 0..10 {
            log::info!(
                "async|test job do something... ; log_id:{}, step:{}",
                &context.log_id,
                i
            );
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        log::info!(
            "async|DemoJobHandler job process done; job_id:{}, log_id:{}",
            &context.job_id,
            &context.log_id
        );
        Ok(context)
    }
}

impl SyncJobHandler for DemoJobHandler {
    fn process(&self, context: JobContext) -> anyhow::Result<JobContext> {
        log::info!(
            "sync|DemoJobHandler job info; job_id:{}, log_id:{}, job_param:{:?}",
            &context.job_id,
            &context.log_id,
            &context.job_param
        );
        for i in 0..15 {
            log::info!(
                "sync|test job do something... ; log_id:{}, step:{}",
                &context.log_id,
                i
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        log::info!(
            "sync|DemoJobHandler job process done; job_id:{}, log_id:{}",
            &context.job_id,
            &context.log_id
        );
        Ok(context)
    }
}
```

（一个对象可同时实现异步、同步处理器接口，由注册时确定使用哪类处理器）


#### 4. 注册任务处理器




```rust
{
        client.register(
            Arc::new("demoJobHandler".to_owned()),
            JobHandler::Async(Arc::new(DemoJobHandler {})),
            //JobHandler::Sync(Arc::new(DemoJobHandler {})),
        )?;

        register_handle(
            Arc::new("demoJobHandler2".to_owned()),
            //JobHandler::Async(Arc::new(DemoJobHandler {})),
            JobHandler::Sync(Arc::new(DemoJobHandler {})),
        )?;
}
// ....


fn register_handle(handle_name: Arc<String>, job_handler: JobHandler) -> anyhow::Result<()> {
    // 获取最近构建的xxl_client全局对象,方便支持构建与注册解耦；
    if let Some(client) = get_last_xxl_client() {
        client.register(handle_name, job_handler)
    } else {
        Err(anyhow::anyhow!("failed to get client"))
    }
}


```

注册任务后即可执行从服务端发起的任务调度。

#### 5. 运行客户端执行器验证功能

需要与服务端配合使用，略。



## 例子

例子完整依赖与代码可以参考 examples/下的代码。


```rust
use async_trait::async_trait;
use std::sync::Arc;
use xxljob_sdk_rs::{get_last_xxl_client, XxlClientBuilder};
use xxljob_sdk_rs::{AsyncJobHandler, JobContext, JobHandler, SyncJobHandler};

pub struct DemoJobHandler;

#[async_trait]
impl AsyncJobHandler for DemoJobHandler {
    async fn process(&self, context: JobContext) -> anyhow::Result<JobContext> {
        log::info!(
            "async|DemoJobHandler job info; job_id:{}, log_id:{}, job_param:{:?}",
            &context.job_id,
            &context.log_id,
            &context.job_param
        );
        for i in 0..10 {
            log::info!(
                "async|test job do something... ; log_id:{}, step:{}",
                &context.log_id,
                i
            );
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        log::info!(
            "async|DemoJobHandler job process done; job_id:{}, log_id:{}",
            &context.job_id,
            &context.log_id
        );
        Ok(context)
    }
}

impl SyncJobHandler for DemoJobHandler {
    fn process(&self, context: JobContext) -> anyhow::Result<JobContext> {
        log::info!(
            "sync|DemoJobHandler job info; job_id:{}, log_id:{}, job_param:{:?}",
            &context.job_id,
            &context.log_id,
            &context.job_param
        );
        for i in 0..15 {
            log::info!(
                "sync|test job do something... ; log_id:{}, step:{}",
                &context.log_id,
                i
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        log::info!(
            "sync|DemoJobHandler job process done; job_id:{}, log_id:{}",
            &context.job_id,
            &context.log_id
        );
        Ok(context)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    log::info!("registry start");
    {
        let client = XxlClientBuilder::new("http://127.0.0.1:8080/xxl-job-admin".to_string())
            .set_access_token("default_token".to_string())
            .set_log_path("xxl-rs-logs".to_string())
            .set_app_name("xxl-job-executor-sample".to_string())
            .build()?;
        client.register(
            Arc::new("demoJobHandler".to_owned()),
            JobHandler::Async(Arc::new(DemoJobHandler {})),
            //JobHandler::Sync(Arc::new(DemoJobHandler {})),
        )?;

        register_handle(
            Arc::new("demoJobHandler2".to_owned()),
            //JobHandler::Async(Arc::new(DemoJobHandler {})),
            JobHandler::Sync(Arc::new(DemoJobHandler {})),
        )?;

        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for event");
    }
    // wait for unregister
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(())
}

fn register_handle(handle_name: Arc<String>, job_handler: JobHandler) -> anyhow::Result<()> {
    // 获取最近构建的xxl_client全局对象,方便支持构建与注册解耦；
    if let Some(client) = get_last_xxl_client() {
        client.register(handle_name, job_handler)
    } else {
        Err(anyhow::anyhow!("failed to get client"))
    }
}

```

