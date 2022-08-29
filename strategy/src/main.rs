enum Currencies {
    USD(f32),
    EUR(f32),
    RUB(f32),
}

trait Converter {
    fn convert(&self, currency: Currencies) -> f32;
}

struct Currency;

impl Currency {
    fn exchange<T: Converter>(coin: T, currency: Currencies) -> f32 {
        coin.convert(currency)
    }
}

struct BTC;
impl Converter for BTC {
    fn convert(&self, currency: Currencies) -> f32 {
        match currency {
            Currencies::USD(usd) => usd / 19_977.90,
            Currencies::EUR(eur) => eur / 20_046.82,
            Currencies::RUB(rub) => rub / 1_204_341.75,
        }
    }
}

struct ETH;
impl Converter for ETH {
    fn convert(&self, currency: Currencies) -> f32 {
        match currency {
            Currencies::USD(usd) => usd / 1481.16,
            Currencies::EUR(eur) => eur / 1486.27,
            Currencies::RUB(rub) => rub / 89_313.95,
        }
    }
}

fn main() {
    let usd_to_btc = Currency::exchange(BTC, Currencies::USD(2345.0));
    let eur_to_btc = Currency::exchange(BTC, Currencies::EUR(4634.0));
    let rub_to_btc = Currency::exchange(BTC, Currencies::RUB(89323.0));

    println!("USD to BTC {}", usd_to_btc);
    println!("EUR to BTC {}", eur_to_btc);
    println!("RUB to BTC {}", rub_to_btc);

    let usd_to_eth = Currency::exchange(ETH, Currencies::USD(2552.0));
    let eur_to_eth = Currency::exchange(ETH, Currencies::EUR(7904.0));
    let rub_to_eth = Currency::exchange(ETH, Currencies::RUB(45432.0));

    println!("USD to ETH {}", usd_to_eth);
    println!("EUR to ETH {}", eur_to_eth);
    println!("RUB to ETH {}", rub_to_eth);
}

#[cfg(test)]
mod tests {
    use crate::{Currencies, Currency, BTC, ETH};
    #[test]
    fn it_should_convert_to_btc() {
        let usd_to_btc = Currency::exchange(BTC, Currencies::USD(1230.0));
        let eur_to_btc = Currency::exchange(BTC, Currencies::EUR(5432.0));
        let rub_to_btc = Currency::exchange(BTC, Currencies::RUB(60000.0));

        assert_eq!(usd_to_btc, 0.061568033);
        assert_eq!(eur_to_btc, 0.27096567);
        assert_eq!(rub_to_btc, 0.049819745);
    }
    #[test]
    fn it_should_convert_eth() {
        let usd_to_eth = Currency::exchange(ETH, Currencies::USD(1230.0));
        let eur_to_eth = Currency::exchange(ETH, Currencies::EUR(5432.0));
        let rub_to_eth = Currency::exchange(ETH, Currencies::RUB(60000.0));

        assert_eq!(usd_to_eth, 0.8304302);
        assert_eq!(eur_to_eth, 3.6547868);
        assert_eq!(rub_to_eth, 0.67178756);
    }
}
