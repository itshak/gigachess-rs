| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.27x | 1.80x | 1.55x | 2.50x | 1.81x | 30/30 |
| materialize | 1.07x | 2.19x | 1.69x | 1.74x | 2.72x | 1.95x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.04x | 1.61x | 1.27x | 2.60x | 1.97x | 30/30 |
| perft_full_d3 | 0.95x | 1.44x | 1.66x | 1.60x | 2.25x | 1.76x | 30/30 |
| fen_parse | 1.08x | 1.40x | 1.35x | 1.16x | 1.46x | 1.16x | 27/30 |
| fen_write_native_ep | 1.01x | 5.16x | 10.35x | 1.89x | 28.83x | 2.53x | 30/30 |
| snapshot | 2.00x | 2.00x | 1.00x | 4.67x | 1.00x | 1.34x | 29/30 |
| san_parse_all | 1.03x | 1.93x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.15x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.71x | 1.08x | 3.25x | 1.09x | 1.26x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.13x | N/A | N/A | 6/6 |
| startpos | 1.21x | 1.00x | 79.08x | 148.62x | 171.97x | 141.83x | 4/5 |
| chess960_materialize | 0.99x | 1.76x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.41x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 39.98x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 19.94x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.82x | 2.50x | 1.41x | 3.21x | 2.80x | 10/10 |
| corpus_materialize | 1.00x | 1.47x | 1.36x | 1.14x | 1.81x | 1.21x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.31x | 3.10x | 2.25x | 3.00x | 1.57x | 10/10 |
| replay_trusted_native | 1.01x | 0.73x | 1.30x | 1.28x | 1.82x | 2.07x | 8/10 |
| replay_checked_native | 1.00x | 2.11x | 1.06x | 8.52x | 8.04x | 9.42x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
