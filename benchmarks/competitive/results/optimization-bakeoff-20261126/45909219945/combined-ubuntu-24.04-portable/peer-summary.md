| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 2.90x | 1.67x | 1.45x | 1.94x | 1.43x | 30/30 |
| materialize | 0.92x | 2.11x | 1.74x | 1.81x | 2.58x | 1.93x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.11x | 1.64x | 1.22x | 2.03x | 1.87x | 29/30 |
| perft_full_d3 | 0.99x | 1.36x | 1.56x | 1.40x | 1.88x | 1.59x | 30/30 |
| fen_parse | 1.00x | 1.53x | 1.47x | 1.20x | 1.54x | 1.27x | 30/30 |
| fen_write_native_ep | 1.01x | 2.86x | 9.71x | 1.91x | 26.40x | 2.56x | 30/30 |
| snapshot | 1.00x | 1.76x | 1.25x | 3.26x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 0.93x | 1.79x | N/A | 4.13x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.24x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.22x | 1.46x | 4.33x | 1.36x | 1.59x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 57.03x | 109.96x | 129.69x | 114.47x | 5/5 |
| chess960_materialize | 1.00x | 1.82x | 1.84x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.11x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 41.01x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.10x | 22.09x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 4.74x | 2.40x | 1.31x | 2.42x | 3.33x | 10/10 |
| corpus_materialize | 1.00x | 1.51x | 1.42x | 1.14x | 1.83x | 1.25x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.93x | 2.91x | 2.18x | 2.85x | 1.55x | 10/10 |
| replay_trusted_native | 1.01x | 0.92x | 1.63x | 1.63x | 2.24x | 2.10x | 9/10 |
| replay_checked_native | 1.00x | 2.07x | 1.13x | 8.85x | 8.67x | 9.86x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
