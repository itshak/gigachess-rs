| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.11x | 1.44x | 1.41x | 2.04x | 1.82x | 30/30 |
| materialize | 1.07x | 1.88x | 1.29x | 1.18x | 2.17x | 1.56x | 30/30 |
| perft_bulk_d4 | 1.38x | 3.84x | 1.62x | 1.47x | 2.35x | 2.12x | 30/30 |
| perft_full_d3 | 2.05x | 1.27x | 1.36x | 1.24x | 1.81x | 1.80x | 30/30 |
| fen_parse | 1.20x | 1.39x | 1.52x | 1.37x | 2.29x | 1.46x | 30/30 |
| fen_write_native_ep | 1.05x | 2.56x | 5.07x | 1.04x | 19.91x | 1.29x | 28/30 |
| snapshot | 1.20x | 1.53x | 1.17x | 2.40x | 1.17x | 1.01x | 29/30 |
| san_parse_all | 2.06x | 1.91x | N/A | 5.25x | N/A | N/A | 12/12 |
| san_render_native | 1.93x | 10.66x | N/A | 3.66x | N/A | N/A | 12/12 |
| apply_children_full | 1.46x | 1.62x | 1.00x | 6.62x | 1.05x | 1.15x | 26/30 |
| make_unmake_full | 1.29x | N/A | N/A | 1.07x | N/A | N/A | 5/6 |
| startpos | 1.70x | 1.11x | 68.02x | 136.88x | 208.73x | 142.22x | 5/5 |
| chess960_materialize | 1.18x | 1.46x | 1.22x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.40x | 2.48x | 1.30x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.46x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.94x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.40x | 19.97x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 5.38x | 1.78x | 1.36x | 2.47x | 2.39x | 10/10 |
| corpus_materialize | 1.05x | 1.61x | 1.28x | 1.14x | 2.03x | 1.43x | 10/10 |
| corpus_uci_sorted | 1.35x | 1.20x | 1.51x | 1.27x | 1.49x | 0.96x | 8/10 |
| replay_trusted_native | 1.31x | 0.59x | 1.15x | 1.47x | 1.86x | 2.21x | 8/10 |
| replay_checked_native | 1.28x | 1.98x | 0.90x | 6.72x | 6.65x | 8.05x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
