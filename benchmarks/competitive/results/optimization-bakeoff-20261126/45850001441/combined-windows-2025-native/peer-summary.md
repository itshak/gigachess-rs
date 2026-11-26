| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.55x | 1.87x | 1.62x | 2.35x | 1.93x | 30/30 |
| materialize | 1.00x | 2.09x | 1.43x | 1.58x | 2.51x | 1.65x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.84x | 1.55x | 1.18x | 2.16x | 1.86x | 29/30 |
| perft_full_d3 | 1.24x | 2.26x | 1.92x | 1.79x | 2.51x | 2.09x | 30/30 |
| fen_parse | 0.97x | 1.27x | 1.53x | 1.29x | 1.80x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.14x | 7.89x | 1.62x | 29.91x | 2.04x | 30/30 |
| snapshot | 0.84x | 2.04x | 1.13x | 4.91x | 1.15x | 1.09x | 29/30 |
| san_parse_all | 1.02x | 1.97x | N/A | 5.28x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.80x | N/A | 4.08x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.05x | 1.34x | 10.80x | 1.33x | 1.64x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.00x | 0.99x | 77.45x | 154.40x | 213.41x | 162.78x | 4/5 |
| chess960_materialize | 1.05x | 1.63x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.57x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.12x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 28.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.12x | 4.86x | 1.95x | 0.98x | 2.18x | 2.05x | 8/10 |
| corpus_materialize | 1.01x | 1.41x | 1.23x | 1.07x | 1.78x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.06x | 1.77x | 2.20x | 1.84x | 2.13x | 1.51x | 10/10 |
| replay_trusted_native | 1.06x | 0.85x | 1.61x | 1.73x | 2.25x | 2.49x | 9/10 |
| replay_checked_native | 1.26x | 2.04x | 1.06x | 8.30x | 7.99x | 9.41x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
