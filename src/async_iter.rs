use async_trait::async_trait;
use futures::stream::{self, BoxStream};

#[async_trait]
pub trait AsyncIter {
    type Item;
    async fn next(&mut self) -> Option<Self::Item>;

    fn into_stream<'a>(self) -> BoxStream<'a, Self::Item>
    where
        Self: Sized + Send + 'a,
    {
        Box::pin(stream::unfold(self, |mut iter| async {
            let value = iter.next().await?;
            Some((value, iter))
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::{executor, StreamExt};

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
