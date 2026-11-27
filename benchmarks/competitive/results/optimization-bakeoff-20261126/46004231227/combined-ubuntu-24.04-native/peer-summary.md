| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.23x | 1.82x | 1.54x | 2.32x | 1.88x | 30/30 |
| materialize | 0.96x | 2.12x | 1.60x | 1.71x | 2.51x | 1.85x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.69x | 1.52x | 1.15x | 2.25x | 1.84x | 29/30 |
| perft_full_d3 | 1.00x | 1.33x | 1.58x | 1.48x | 1.93x | 1.66x | 30/30 |
| fen_parse | 0.93x | 1.32x | 1.28x | 1.07x | 1.37x | 1.06x | 30/30 |
| fen_write_native_ep | 1.00x | 4.69x | 9.88x | 1.92x | 28.20x | 2.53x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.01x | 1.94x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.20x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.13x | 1.34x | 4.07x | 1.27x | 1.75x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.11x | 1.24x | 99.03x | 185.84x | 210.37x | 173.78x | 5/5 |
| chess960_materialize | 0.99x | 1.74x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.51x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.53x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.23x | 24.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.26x | 1.98x | 1.09x | 2.44x | 2.23x | 9/10 |
| corpus_materialize | 1.01x | 1.41x | 1.30x | 1.07x | 1.71x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.25x | 2.86x | 2.27x | 2.82x | 1.56x | 10/10 |
| replay_trusted_native | 1.02x | 0.90x | 1.63x | 1.58x | 2.24x | 2.96x | 9/10 |
| replay_checked_native | 0.99x | 2.18x | 1.14x | 9.06x | 8.68x | 10.42x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
