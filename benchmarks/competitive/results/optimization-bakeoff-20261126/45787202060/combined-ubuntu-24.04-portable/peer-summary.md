| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 2.56x | 1.48x | 1.29x | 1.74x | 1.26x | 29/30 |
| materialize | 0.98x | 2.11x | 1.73x | 1.75x | 2.61x | 1.91x | 30/30 |
| perft_bulk_d4 | 0.86x | 2.55x | 1.35x | 1.00x | 1.68x | 1.52x | 28/30 |
| perft_full_d3 | 0.87x | 1.35x | 1.56x | 1.41x | 1.90x | 1.57x | 30/30 |
| fen_parse | 0.88x | 1.35x | 1.22x | 1.08x | 1.27x | 1.09x | 28/30 |
| fen_write_native_ep | 1.01x | 3.18x | 10.33x | 1.82x | 28.40x | 2.59x | 30/30 |
| snapshot | 1.01x | 1.63x | 1.25x | 3.37x | 1.25x | 1.12x | 30/30 |
| san_parse_all | 0.93x | 1.79x | N/A | 3.95x | N/A | N/A | 12/12 |
| san_render_native | 0.86x | 8.24x | N/A | 1.89x | N/A | N/A | 12/12 |
| apply_children_full | 0.87x | 1.56x | 1.05x | 2.99x | 0.99x | 1.12x | 25/30 |
| make_unmake_full | 0.68x | N/A | N/A | 0.81x | N/A | N/A | 0/6 |
| startpos | 1.17x | 1.58x | 71.52x | 125.83x | 139.18x | 120.06x | 5/5 |
| chess960_materialize | 0.99x | 1.54x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.81x | 1.52x | 1.08x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 41.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 20.73x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.56x | 2.31x | 1.22x | 2.39x | 3.19x | 10/10 |
| corpus_materialize | 1.02x | 1.52x | 1.43x | 1.16x | 1.90x | 1.27x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.94x | 3.06x | 2.21x | 2.96x | 1.55x | 10/10 |
| replay_trusted_native | 0.98x | 0.70x | 1.33x | 1.29x | 1.75x | 1.64x | 8/10 |
| replay_checked_native | 0.74x | 1.47x | 0.83x | 6.29x | 6.05x | 6.96x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
