#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod api;

use eframe::egui;
use models::{BlackScholes, Greeks};
use std::sync::mpsc::{channel, Receiver, Sender};
use tokio::runtime::Runtime;

enum MarketData {
    Price(f64),
    Vol(f64),
    Selic(f64),
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 650.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Black-Scholes Quant Terminal",
        options,
        Box::new(|_cc| Box::new(BSApp::new())),
    )
}

struct BSApp {
    ticker: String,
    s: f64, k: f64, t_days: f64, r: f64, sigma: f64,
    prices: (f64, f64),
    greeks: Greeks,
    tx: Sender<MarketData>,
    rx: Receiver<MarketData>,
    rt: Runtime,
}

impl BSApp {
    fn new() -> Self {
        let (tx, rx) = channel();
        let rt = Runtime::new().unwrap();
        let tx_s = tx.clone();
        rt.spawn(async move {
            if let Some(s) = api::fetch_selic().await { let _ = tx_s.send(MarketData::Selic(s)); }
        });

        Self {
            ticker: "PETR4".into(),
            s: 35.0, k: 35.0, t_days: 21.0, r: 10.75, sigma: 30.0,
            prices: (0.0, 0.0),
            greeks: Greeks { delta: 0.0, gamma: 0.0, theta: 0.0, vega: 0.0 },
            tx, rx, rt,
        }
    }
}

impl eframe::App for BSApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(data) = self.rx.try_recv() {
            match data {
                MarketData::Price(p) => self.s = p,
                MarketData::Vol(v) => self.sigma = v,
                MarketData::Selic(r) => self.r = r,
            }
        }

        // Cálculo Reativo Profissional
        let calc = BlackScholes {
            s: self.s, k: self.k,
            t: self.t_days / 252.0,
            r: (1.0 + self.r / 100.0).ln(), // Conversão para taxa contínua
            sigma: self.sigma / 100.0,
        };
        let (c, p, g) = calc.calculate();
        self.prices = (c, p);
        self.greeks = g;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading("📊 Opções & Risco - Black-Scholes"));
            
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Ticker:");
                    ui.text_edit_singleline(&mut self.ticker);
                    if ui.button("Importar Mercado").clicked() {
                        let tx = self.tx.clone();
                        let t = self.ticker.clone();
                        let ctx_c = ctx.clone();
                        self.rt.spawn(async move {
                            if let Some((p, v)) = api::fetch_market_data(&t).await {
                                let _ = tx.send(MarketData::Price(p));
                                let _ = tx.send(MarketData::Vol(v));
                                ctx_c.request_repaint();
                            }
                        });
                    }
                });
            });

            ui.add_space(10.0);
            egui::Grid::new("in").spacing([20.0, 8.0]).show(ui, |ui| {
                ui.label("Ativo (S):"); ui.add(egui::DragValue::new(&mut self.s).speed(0.05)); ui.end_row();
                ui.label("Strike (K):"); ui.add(egui::DragValue::new(&mut self.k).speed(0.05)); ui.end_row();
                ui.label("Dias Úteis:"); ui.add(egui::DragValue::new(&mut self.t_days).speed(1.0).clamp_range(1.0..=1000.0)); ui.end_row();
                ui.label("Juros %:"); ui.add(egui::DragValue::new(&mut self.r).speed(0.01)); ui.end_row();
                ui.label("Vol %:"); ui.add(egui::DragValue::new(&mut self.sigma).speed(0.1)); ui.end_row();
            });

            ui.add_space(15.0);
            ui.separator();
            ui.columns(2, |cols| {
                cols[0].vertical_centered(|ui| { ui.label("CALL"); ui.heading(format!("R$ {:.2}", self.prices.0)); });
                cols[1].vertical_centered(|ui| { ui.label("PUT"); ui.heading(format!("R$ {:.2}", self.prices.1)); });
            });

            ui.add_space(15.0);
            ui.group(|ui| {
                ui.label("Gregas (Risco da Call)");
                egui::Grid::new("greeks").spacing([40.0, 5.0]).show(ui, |ui| {
                    ui.label(format!("Delta: {:.3}", self.greeks.delta));
                    ui.label(format!("Gamma: {:.4}", self.greeks.gamma));
                    ui.end_row();
                    ui.label(format!("Theta: {:.3}/dia", self.greeks.theta));
                    ui.label(format!("Vega: {:.3}/1% vol", self.greeks.vega));
                });
            });
            ui.vertical_centered(|ui| ui.weak("Simule variações nos parâmetros para ver o risco mudar."));
        });
    }
}