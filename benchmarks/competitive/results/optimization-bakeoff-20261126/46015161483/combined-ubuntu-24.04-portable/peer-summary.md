| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 3.33x | 1.73x | 1.49x | 2.12x | 1.55x | 30/30 |
| materialize | 0.98x | 2.21x | 1.86x | 1.29x | 2.56x | 1.60x | 30/30 |
| perft_bulk_d4 | 1.08x | 3.35x | 1.72x | 1.31x | 2.12x | 1.94x | 30/30 |
| perft_full_d3 | 1.25x | 1.77x | 2.22x | 1.72x | 2.43x | 2.22x | 30/30 |
| fen_parse | 0.90x | 1.51x | 1.50x | 1.17x | 1.87x | 1.31x | 29/30 |
| fen_write_native_ep | 1.00x | 2.73x | 8.54x | 1.53x | 25.06x | 1.99x | 30/30 |
| snapshot | 1.01x | 1.54x | 1.16x | 2.64x | 1.15x | 1.00x | 27/30 |
| san_parse_all | 1.01x | 1.78x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.94x | N/A | 2.33x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.19x | 1.49x | 5.14x | 1.29x | 1.33x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.29x | 58.46x | 80.52x | 118.45x | 84.29x | 5/5 |
| chess960_materialize | 1.02x | 1.83x | 1.80x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.37x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 32.02x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.30x | 1.32x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 25.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.12x | 4.68x | 1.99x | 1.21x | 2.11x | 3.09x | 9/10 |
| corpus_materialize | 1.01x | 1.76x | 1.63x | 1.21x | 2.30x | 1.46x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.04x | 2.79x | 2.28x | 2.85x | 1.59x | 10/10 |
| replay_trusted_native | 1.01x | 0.74x | 1.69x | 1.69x | 2.27x | 2.07x | 8/10 |
| replay_checked_native | 1.01x | 2.26x | 1.28x | 8.62x | 8.63x | 10.37x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
