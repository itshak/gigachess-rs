| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.02x | 1.77x | 1.47x | 2.01x | 1.80x | 30/30 |
| materialize | 0.98x | 1.94x | 1.32x | 1.54x | 2.28x | 1.60x | 30/30 |
| perft_bulk_d4 | 0.95x | 3.72x | 1.54x | 1.21x | 2.12x | 1.83x | 30/30 |
| perft_full_d3 | 0.92x | 0.83x | 0.94x | 0.85x | 1.14x | 0.99x | 11/30 |
| fen_parse | 0.96x | 1.12x | 1.31x | 1.23x | 1.62x | 1.31x | 30/30 |
| fen_write_native_ep | 0.97x | 3.18x | 6.32x | 1.33x | 22.96x | 1.72x | 30/30 |
| snapshot | 0.84x | 1.58x | 1.10x | 2.83x | 1.20x | 1.04x | 30/30 |
| san_parse_all | 0.95x | 1.58x | N/A | 4.96x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.49x | N/A | 4.08x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.47x | 0.96x | 7.22x | 0.95x | 1.15x | 22/30 |
| make_unmake_full | 0.98x | N/A | N/A | 0.95x | N/A | N/A | 2/6 |
| startpos | 1.02x | 1.09x | 66.87x | 141.95x | 178.72x | 145.38x | 5/5 |
| chess960_materialize | 0.99x | 1.51x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.26x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.30x | 44.28x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 25.20x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 6.32x | 2.59x | 1.24x | 2.76x | 2.84x | 10/10 |
| corpus_materialize | 1.00x | 1.41x | 1.23x | 1.10x | 1.81x | 1.26x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.20x | 1.37x | 1.17x | 1.31x | 0.96x | 8/10 |
| replay_trusted_native | 0.97x | 0.56x | 1.05x | 1.09x | 1.47x | 1.66x | 7/10 |
| replay_checked_native | 0.96x | 1.85x | 0.94x | 7.34x | 6.74x | 8.73x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
