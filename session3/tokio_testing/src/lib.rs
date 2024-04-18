pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub async fn double(n: u32) -> u32 {
    n * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn will_compile() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        assert_eq!(rt.block_on(double(2)), 4);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn the_easy_way() {
        assert_eq!(double(2).await, 4);
    }
}
