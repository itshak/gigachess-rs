| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.82x | 1.77x | 1.72x | 2.69x | 2.18x | 30/30 |
| materialize | 1.00x | 2.02x | 1.36x | 1.16x | 2.34x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.01x | 1.64x | 1.45x | 2.46x | 2.27x | 30/30 |
| perft_full_d3 | 1.01x | 1.52x | 1.58x | 1.26x | 2.14x | 1.90x | 30/30 |
| fen_parse | 1.00x | 1.47x | 1.56x | 1.43x | 2.13x | 1.60x | 30/30 |
| fen_write_native_ep | 1.02x | 3.98x | 7.96x | 1.64x | 31.58x | 2.02x | 30/30 |
| snapshot | 1.01x | 1.80x | 1.25x | 4.10x | 1.27x | 1.17x | 30/30 |
| san_parse_all | 1.00x | 2.01x | N/A | 4.82x | N/A | N/A | 12/12 |
| san_render_native | 1.08x | 9.97x | N/A | 3.60x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.95x | 1.32x | 8.21x | 1.54x | 1.75x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.01x | 86.17x | 132.50x | 199.39x | 154.53x | 5/5 |
| chess960_materialize | 1.00x | 1.67x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.93x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 18.94x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.44x | 1.79x | 1.32x | 2.38x | 2.15x | 9/10 |
| corpus_materialize | 1.00x | 1.77x | 1.37x | 1.20x | 2.85x | 1.52x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.90x | 2.40x | 1.99x | 2.28x | 1.53x | 10/10 |
| replay_trusted_native | 0.99x | 1.07x | 1.92x | 2.27x | 3.61x | 4.56x | 10/10 |
| replay_checked_native | 1.02x | 2.49x | 1.13x | 6.18x | 7.33x | 8.97x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
