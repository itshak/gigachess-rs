| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.49x | 1.89x | 1.63x | 2.63x | 1.92x | 30/30 |
| materialize | 1.10x | 2.27x | 1.68x | 1.78x | 2.87x | 1.98x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.81x | 1.52x | 1.20x | 2.41x | 1.86x | 29/30 |
| perft_full_d3 | 0.99x | 1.27x | 1.42x | 1.44x | 1.83x | 1.52x | 30/30 |
| fen_parse | 1.08x | 1.26x | 1.40x | 1.18x | 1.64x | 1.17x | 30/30 |
| fen_write_native_ep | 0.99x | 4.59x | 10.31x | 1.92x | 29.91x | 2.20x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.01x | 1.34x | 1.33x | 30/30 |
| san_parse_all | 0.90x | 1.87x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.27x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.11x | 1.33x | 4.04x | 1.25x | 2.28x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.11x | N/A | N/A | 5/6 |
| startpos | 1.13x | 1.25x | 103.95x | 184.87x | 231.29x | 178.16x | 5/5 |
| chess960_materialize | 1.06x | 1.72x | 1.65x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.52x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 39.85x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.11x | 1.10x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.21x | 23.86x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.46x | 2.00x | 1.14x | 2.48x | 2.28x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.31x | 1.10x | 1.74x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.28x | 3.06x | 2.26x | 2.96x | 1.58x | 10/10 |
| replay_trusted_native | 0.99x | 0.87x | 1.54x | 1.62x | 2.22x | 2.56x | 9/10 |
| replay_checked_native | 0.97x | 1.95x | 1.02x | 8.34x | 7.95x | 9.32x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
