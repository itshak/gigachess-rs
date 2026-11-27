| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.17x | 4.93x | 1.55x | 1.65x | 2.41x | 1.95x | 29/30 |
| materialize | 1.23x | 2.07x | 1.67x | 1.06x | 2.31x | 1.50x | 28/30 |
| perft_bulk_d4 | 1.18x | 3.88x | 1.49x | 1.26x | 1.83x | 1.74x | 29/30 |
| perft_full_d3 | 1.12x | 1.51x | 1.75x | 1.30x | 1.84x | 1.93x | 30/30 |
| fen_parse | 1.08x | 1.20x | 1.46x | 1.31x | 1.60x | 1.45x | 30/30 |
| fen_write_native_ep | 1.16x | 4.39x | 9.32x | 1.86x | 29.95x | 2.48x | 30/30 |
| snapshot | 1.31x | 1.79x | 1.18x | 2.73x | 1.31x | 1.08x | 27/30 |
| san_parse_all | 1.04x | 1.77x | N/A | 3.79x | N/A | N/A | 12/12 |
| san_render_native | 1.12x | 13.30x | N/A | 2.78x | N/A | N/A | 12/12 |
| apply_children_full | 1.22x | 1.11x | 1.30x | 5.85x | 1.22x | 1.36x | 29/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.22x | N/A | N/A | 5/6 |
| startpos | 1.01x | 1.15x | 69.38x | 135.54x | 167.02x | 139.95x | 5/5 |
| chess960_materialize | 1.11x | 1.84x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.21x | 3.20x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.19x | 41.60x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.08x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 20.26x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.29x | 6.31x | 2.07x | 1.32x | 2.56x | 2.61x | 10/10 |
| corpus_materialize | 1.10x | 1.51x | 1.40x | 1.16x | 2.00x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.13x | 1.82x | 2.65x | 1.93x | 2.68x | 1.40x | 10/10 |
| replay_trusted_native | 1.04x | 0.67x | 1.82x | 1.76x | 1.92x | 1.94x | 8/10 |
| replay_checked_native | 1.16x | 2.82x | 1.28x | 10.04x | 8.28x | 10.36x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
