| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.08x | 1.67x | 1.42x | 2.19x | 1.71x | 30/30 |
| materialize | 0.99x | 2.10x | 1.65x | 1.65x | 2.54x | 1.79x | 30/30 |
| perft_bulk_d4 | 0.86x | 3.30x | 1.36x | 1.03x | 2.03x | 1.61x | 27/30 |
| perft_full_d3 | 0.94x | 1.42x | 1.64x | 1.59x | 2.12x | 1.75x | 30/30 |
| fen_parse | 0.96x | 1.40x | 1.24x | 1.08x | 1.35x | 1.17x | 29/30 |
| fen_write_native_ep | 1.01x | 4.53x | 10.11x | 1.92x | 28.10x | 2.54x | 30/30 |
| snapshot | 1.42x | 1.83x | 1.15x | 4.32x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 0.98x | 1.93x | N/A | 4.36x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.12x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.76x | 1.11x | 3.35x | 1.11x | 1.40x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.09x | 1.12x | 89.12x | 160.63x | 194.28x | 174.54x | 5/5 |
| chess960_materialize | 1.05x | 1.57x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.34x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 38.88x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.80x | 16.51x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.65x | 4.22x | 1.61x | 0.87x | 1.93x | 1.76x | 8/10 |
| corpus_materialize | 0.95x | 1.35x | 1.26x | 1.05x | 1.65x | 1.13x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.27x | 2.98x | 2.22x | 2.92x | 1.57x | 10/10 |
| replay_trusted_native | 1.01x | 0.71x | 1.29x | 1.31x | 1.78x | 2.10x | 8/10 |
| replay_checked_native | 1.00x | 1.99x | 1.11x | 8.65x | 8.16x | 9.83x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
