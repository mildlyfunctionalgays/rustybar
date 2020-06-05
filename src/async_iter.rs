use async_trait::async_trait;
use futures::stream::BoxStream;
use futures::{stream };

#[async_trait]
trait AsyncIter {
    type Item: Sized;
    async fn next(&mut self) -> Option<Self::Item>;

    fn into_stream(self) -> BoxStream<'static, Self::Item>
    where
        Self: Sized + Send + 'static,
    {
        async fn helper<I>(mut iter: I) -> Option<(I::Item, I)>
        where
            I: AsyncIter,
        {
            let value = iter.next().await?;
            Some((value, iter))
        }
        Box::pin(stream::unfold(self, helper))
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use futures::StreamExt;
    use futures::executor;

    struct Numbers(usize);
    impl Numbers {
        fn new() -> Self {
            Numbers(0)
        }
    }

    #[async_trait]
    impl AsyncIter for Numbers {
        type Item = usize;

        async fn next(&mut self) -> Option<Self::Item> {
            self.0 = self.0.checked_add(1)?;
            Some(self.0)
        }
    }
    #[test]
    fn test_numbers() {
        let a = async {
            let vec1: Vec<_> = Numbers::new().into_stream().take(10).collect().await;
            let vec2: Vec<_> = (1..=10).into_iter().collect();
            assert_eq!(vec1, vec2);
        };
        executor::block_on(a);
    }
}
