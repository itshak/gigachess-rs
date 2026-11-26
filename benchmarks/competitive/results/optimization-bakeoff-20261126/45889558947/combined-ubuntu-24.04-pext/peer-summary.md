| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.51x | 1.88x | 1.64x | 2.57x | 1.92x | 30/30 |
| materialize | 1.00x | 2.28x | 1.68x | 1.75x | 2.65x | 1.99x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.82x | 1.52x | 1.18x | 2.36x | 1.87x | 29/30 |
| perft_full_d3 | 1.00x | 1.34x | 1.56x | 1.50x | 1.99x | 1.64x | 30/30 |
| fen_parse | 0.98x | 1.32x | 1.55x | 1.02x | 1.42x | 1.22x | 27/30 |
| fen_write_native_ep | 1.01x | 4.62x | 10.08x | 2.15x | 27.78x | 2.68x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.67x | 1.00x | 1.33x | 22/30 |
| san_parse_all | 1.00x | 1.88x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 8.95x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.15x | 1.35x | 4.05x | 1.44x | 1.65x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.00x | 78.54x | 138.09x | 175.44x | 159.11x | 5/5 |
| chess960_materialize | 1.01x | 1.85x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.52x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 38.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.07x | 1.06x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 22.65x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.48x | 2.03x | 1.13x | 2.50x | 2.27x | 10/10 |
| corpus_materialize | 1.00x | 1.42x | 1.30x | 1.08x | 1.73x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.32x | 3.04x | 2.29x | 2.93x | 1.61x | 10/10 |
| replay_trusted_native | 1.00x | 0.91x | 1.65x | 1.58x | 2.23x | 2.59x | 9/10 |
| replay_checked_native | 1.00x | 2.18x | 1.18x | 9.19x | 8.87x | 10.48x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
