use anyhow::Result;
use high_performance_backtester::*;
use rust_decimal_macros::dec;

fn main() -> Result<()> {
    println!("=== High-Performance Backtester - Simple Strategy ===\n");

    // Create sample trades
    let trades = vec![
        Trade {
            symbol: "BTC/USD".to_string(),
            side: "BUY".to_string(),
            quantity: dec!(1.0),
            price: dec!(45000.00),
            timestamp: chrono::Utc::now(),
        },
        Trade {
            symbol: "BTC/USD".to_string(),
            side: "SELL".to_string(),
            quantity: dec!(1.0),
            price: dec!(48000.00),
            timestamp: chrono::Utc::now(),
        },
    ];

    println!("📊 Backtest Results:");
    println!("  Total Trades: {}", trades.len());

    // Calculate simple P&L
    let mut total_pnl = dec!(0);
    for trade in &trades {
        println!("\n  Trade:");
        println!("    Symbol: {}", trade.symbol);
        println!("    Side: {}", trade.side);
        println!("    Quantity: {}", trade.quantity);
        println!("    Price: ${}", trade.price);

        if trade.side == "SELL" {
            total_pnl += trade.price * trade.quantity;
        } else {
            total_pnl -= trade.price * trade.quantity;
        }
    }

    println!("\n  📈 Performance:");
    println!("    Total P&L: ${}", total_pnl);
    println!("    Return: {}%", (total_pnl / dec!(45000.00)) * dec!(100));

    // Create sample backtest result
    let result = BacktestResult {
        total_return: dec!(6.67),
        sharpe_ratio: dec!(1.85),
        max_drawdown: dec!(-2.5),
        win_rate: dec!(75.0),
    };

    println!("\n  📊 Metrics:");
    println!("    Total Return: {}%", result.total_return);
    println!("    Sharpe Ratio: {}", result.sharpe_ratio);
    println!("    Max Drawdown: {}%", result.max_drawdown);
    println!("    Win Rate: {}%", result.win_rate);

    println!("\n=== Backtest Complete ===");
    Ok(())
}
