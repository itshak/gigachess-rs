| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.38x | 1.52x | 1.54x | 2.38x | 1.76x | 30/30 |
| materialize | 1.09x | 2.09x | 1.66x | 1.16x | 2.19x | 1.42x | 30/30 |
| perft_bulk_d4 | 1.18x | 3.83x | 1.48x | 1.23x | 2.32x | 1.93x | 30/30 |
| perft_full_d3 | 1.00x | 0.79x | 0.82x | 0.62x | 1.13x | 0.91x | 7/30 |
| fen_parse | 1.12x | 1.32x | 1.56x | 1.12x | 1.93x | 1.37x | 30/30 |
| fen_write_native_ep | 1.01x | 2.83x | 6.60x | 1.23x | 18.48x | 1.43x | 30/30 |
| snapshot | 1.08x | 1.43x | 0.83x | 2.70x | 0.84x | 0.83x | 12/30 |
| san_parse_all | 1.42x | 1.44x | N/A | 2.53x | N/A | N/A | 12/12 |
| san_render_native | 1.48x | 8.11x | N/A | 1.68x | N/A | N/A | 12/12 |
| apply_children_full | 1.12x | 1.13x | 0.75x | 2.70x | 0.87x | 1.00x | 15/30 |
| make_unmake_full | 1.07x | N/A | N/A | 0.90x | N/A | N/A | 0/6 |
| startpos | 1.06x | 0.75x | 58.51x | 86.85x | 131.83x | 99.12x | 4/5 |
| chess960_materialize | 1.21x | 1.75x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.30x | 2.55x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 22.37x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.05x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 13.63x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.05x | 4.74x | 1.81x | 1.39x | 2.51x | 2.07x | 10/10 |
| corpus_materialize | 1.02x | 1.95x | 1.74x | 1.25x | 2.90x | 1.52x | 10/10 |
| corpus_uci_sorted | 1.20x | 1.07x | 1.46x | 1.08x | 1.36x | 0.77x | 8/10 |
| replay_trusted_native | 1.11x | 0.57x | 0.96x | 1.12x | 2.02x | 2.27x | 5/10 |
| replay_checked_native | 1.20x | 2.26x | 0.85x | 4.98x | 5.53x | 6.46x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
