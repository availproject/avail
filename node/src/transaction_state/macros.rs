macro_rules! profile {
    ($($token:tt)+) => {
        {
            let _instant = std::time::Instant::now();
            let _result = {
                $($token)+
            };

            (_instant.elapsed(), _result)
        }
    }
}

pub(crate) use profile;
