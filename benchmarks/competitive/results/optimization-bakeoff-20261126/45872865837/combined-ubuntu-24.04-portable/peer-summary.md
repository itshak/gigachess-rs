| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.17x | 1.72x | 1.47x | 2.07x | 1.45x | 30/30 |
| materialize | 1.00x | 2.12x | 1.89x | 1.38x | 2.50x | 1.63x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.10x | 1.69x | 1.31x | 2.12x | 1.82x | 30/30 |
| perft_full_d3 | 1.20x | 1.64x | 1.89x | 1.52x | 2.28x | 2.08x | 30/30 |
| fen_parse | 1.00x | 1.30x | 1.39x | 1.09x | 1.99x | 1.20x | 29/30 |
| fen_write_native_ep | 1.08x | 2.99x | 9.65x | 1.68x | 27.31x | 2.02x | 30/30 |
| snapshot | 1.00x | 2.51x | 2.22x | 3.05x | 1.01x | 0.97x | 24/30 |
| san_parse_all | 1.00x | 2.03x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.34x | N/A | 2.29x | N/A | N/A | 12/12 |
| apply_children_full | 0.88x | 2.19x | 1.38x | 4.21x | 1.29x | 1.31x | 30/30 |
| make_unmake_full | 0.89x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 64.13x | 110.97x | 175.31x | 113.72x | 5/5 |
| chess960_materialize | 1.00x | 1.69x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.30x | 1.56x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 27.49x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 19.44x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.85x | 1.94x | 1.28x | 2.11x | 2.56x | 10/10 |
| corpus_materialize | 0.99x | 1.59x | 1.61x | 1.20x | 2.08x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.23x | 3.21x | 2.46x | 3.10x | 1.82x | 10/10 |
| replay_trusted_native | 0.80x | 0.72x | 1.44x | 1.58x | 2.13x | 1.99x | 8/10 |
| replay_checked_native | 0.99x | 2.34x | 1.13x | 8.09x | 8.14x | 9.12x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
