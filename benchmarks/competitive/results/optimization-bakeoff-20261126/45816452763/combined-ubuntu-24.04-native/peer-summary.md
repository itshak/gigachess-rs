| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.15x | 1.51x | 1.45x | 2.22x | 1.78x | 30/30 |
| materialize | 1.00x | 1.83x | 1.62x | 1.19x | 2.17x | 1.42x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.49x | 1.41x | 1.25x | 2.12x | 1.74x | 30/30 |
| perft_full_d3 | 0.95x | 1.15x | 1.30x | 1.14x | 1.67x | 1.61x | 29/30 |
| fen_parse | 1.02x | 1.42x | 1.41x | 1.21x | 2.23x | 1.35x | 30/30 |
| fen_write_native_ep | 1.01x | 4.29x | 9.38x | 1.72x | 28.15x | 2.05x | 30/30 |
| snapshot | 1.29x | 1.97x | 1.25x | 3.76x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.02x | 2.10x | N/A | 4.42x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.09x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 0.89x | 1.89x | 1.07x | 3.74x | 1.21x | 1.33x | 27/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.10x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 76.62x | 137.64x | 223.73x | 149.59x | 5/5 |
| chess960_materialize | 0.94x | 1.38x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 2.41x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.65x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 18.51x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.90x | 1.67x | 1.24x | 2.35x | 2.15x | 10/10 |
| corpus_materialize | 1.00x | 1.51x | 1.46x | 1.12x | 1.98x | 1.29x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.46x | 3.18x | 2.42x | 3.06x | 1.79x | 10/10 |
| replay_trusted_native | 0.92x | 0.71x | 1.32x | 1.59x | 2.08x | 2.40x | 8/10 |
| replay_checked_native | 0.96x | 2.22x | 0.96x | 7.60x | 7.33x | 8.68x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
