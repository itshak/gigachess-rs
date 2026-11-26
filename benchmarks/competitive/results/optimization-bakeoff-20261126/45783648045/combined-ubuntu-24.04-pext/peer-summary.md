| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.51x | 1.55x | 1.57x | 2.41x | 1.81x | 30/30 |
| materialize | 1.00x | 1.89x | 1.57x | 1.22x | 2.19x | 1.46x | 30/30 |
| perft_bulk_d4 | 0.98x | 4.05x | 1.53x | 1.34x | 2.37x | 1.87x | 30/30 |
| perft_full_d3 | 0.98x | 1.23x | 1.39x | 1.21x | 1.79x | 1.65x | 30/30 |
| fen_parse | 0.85x | 1.17x | 1.25x | 1.04x | 1.90x | 1.09x | 28/30 |
| fen_write_native_ep | 1.01x | 2.17x | 5.23x | 0.92x | 14.94x | 1.10x | 24/30 |
| snapshot | 1.34x | 2.20x | 1.25x | 3.62x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.03x | 2.13x | N/A | 4.46x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.25x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.07x | 1.19x | 3.90x | 1.27x | 1.58x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.07x | 66.78x | 124.78x | 196.17x | 125.40x | 5/5 |
| chess960_materialize | 1.00x | 1.47x | 1.30x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.67x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 29.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 22.79x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.89x | 1.94x | 1.48x | 2.87x | 2.44x | 10/10 |
| corpus_materialize | 1.01x | 1.67x | 1.60x | 1.20x | 2.10x | 1.41x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.42x | 3.18x | 2.46x | 3.05x | 1.78x | 10/10 |
| replay_trusted_native | 1.02x | 0.73x | 1.32x | 1.57x | 2.11x | 2.39x | 8/10 |
| replay_checked_native | 0.98x | 2.23x | 0.98x | 7.32x | 7.48x | 8.57x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
