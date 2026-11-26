| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 3.73x | 1.54x | 1.36x | 1.91x | 1.60x | 30/30 |
| materialize | 1.00x | 1.98x | 1.59x | 1.60x | 2.31x | 1.59x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.36x | 1.33x | 1.04x | 1.91x | 1.62x | 28/30 |
| perft_full_d3 | 1.00x | 1.52x | 1.71x | 1.61x | 2.14x | 1.98x | 30/30 |
| fen_parse | 1.13x | 1.21x | 1.61x | 1.24x | 1.55x | 1.23x | 30/30 |
| fen_write_native_ep | 1.06x | 5.38x | 10.56x | 2.05x | 28.35x | 2.28x | 30/30 |
| snapshot | 1.00x | 1.84x | 1.16x | 4.33x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 0.95x | 1.73x | N/A | 4.04x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.36x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.08x | 1.32x | 4.35x | 1.32x | 1.91x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.13x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.13x | 92.11x | 162.93x | 195.58x | 160.63x | 5/5 |
| chess960_materialize | 1.00x | 1.51x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.29x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 39.65x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.15x | 24.09x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.17x | 1.60x | 0.83x | 1.88x | 1.79x | 8/10 |
| corpus_materialize | 1.00x | 1.38x | 1.28x | 1.07x | 1.67x | 1.14x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.34x | 2.96x | 2.25x | 2.84x | 1.55x | 10/10 |
| replay_trusted_native | 0.98x | 0.86x | 1.58x | 1.56x | 2.12x | 2.42x | 9/10 |
| replay_checked_native | 1.06x | 2.32x | 1.18x | 9.29x | 8.95x | 10.24x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
