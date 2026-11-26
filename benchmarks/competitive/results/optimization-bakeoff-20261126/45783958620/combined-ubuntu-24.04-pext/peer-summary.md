| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.27x | 1.80x | 1.56x | 2.40x | 1.82x | 30/30 |
| materialize | 1.00x | 2.21x | 1.69x | 1.76x | 2.69x | 1.87x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.27x | 1.72x | 1.27x | 2.50x | 1.96x | 30/30 |
| perft_full_d3 | 0.98x | 2.25x | 2.02x | 1.62x | 2.09x | 1.71x | 30/30 |
| fen_parse | 0.99x | 1.16x | 1.22x | 1.10x | 1.34x | 1.04x | 28/30 |
| fen_write_native_ep | 1.00x | 4.66x | 10.24x | 1.94x | 30.12x | 2.29x | 30/30 |
| snapshot | 1.00x | 1.42x | 1.16x | 3.23x | 1.15x | 1.15x | 30/30 |
| san_parse_all | 1.01x | 1.95x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.02x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.66x | 1.05x | 3.10x | 1.04x | 1.32x | 28/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.06x | N/A | N/A | 4/6 |
| startpos | 1.12x | 1.21x | 81.17x | 149.03x | 174.40x | 141.62x | 5/5 |
| chess960_materialize | 0.97x | 1.70x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.31x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 38.63x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.27x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.83x | 2.51x | 1.37x | 3.12x | 2.84x | 10/10 |
| corpus_materialize | 1.00x | 1.49x | 1.38x | 1.16x | 1.82x | 1.22x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.28x | 3.05x | 2.26x | 2.98x | 1.58x | 10/10 |
| replay_trusted_native | 0.93x | 0.65x | 1.20x | 1.19x | 1.66x | 1.90x | 7/10 |
| replay_checked_native | 0.97x | 1.92x | 1.03x | 8.38x | 7.89x | 9.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
