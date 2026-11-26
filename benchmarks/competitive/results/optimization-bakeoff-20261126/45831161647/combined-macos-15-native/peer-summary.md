| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.45x | 1.37x | 1.44x | 1.92x | 1.79x | 29/30 |
| materialize | 1.01x | 2.19x | 1.61x | 1.03x | 2.46x | 1.45x | 27/30 |
| perft_bulk_d4 | 1.03x | 3.62x | 1.40x | 1.05x | 1.69x | 1.57x | 28/30 |
| perft_full_d3 | 1.07x | 1.54x | 1.69x | 1.30x | 1.85x | 2.03x | 30/30 |
| fen_parse | 1.04x | 1.24x | 1.35x | 1.32x | 1.66x | 1.48x | 30/30 |
| fen_write_native_ep | 0.98x | 4.07x | 8.89x | 1.76x | 27.79x | 2.22x | 30/30 |
| snapshot | 0.95x | 1.62x | 1.05x | 2.31x | 1.08x | 0.95x | 23/30 |
| san_parse_all | 1.06x | 1.69x | N/A | 3.56x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 11.02x | N/A | 2.51x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 1.09x | 1.21x | 5.65x | 1.19x | 1.38x | 27/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.24x | N/A | N/A | 5/6 |
| startpos | 0.74x | 1.23x | 60.75x | 124.66x | 177.12x | 111.38x | 5/5 |
| chess960_materialize | 0.97x | 1.75x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.89x | 2.45x | 1.17x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 38.42x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.88x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 18.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 4.98x | 1.64x | 1.09x | 1.86x | 2.04x | 10/10 |
| corpus_materialize | 0.83x | 1.22x | 1.32x | 0.97x | 1.76x | 1.07x | 8/10 |
| corpus_uci_sorted | 0.94x | 1.80x | 2.83x | 2.17x | 2.66x | 1.48x | 10/10 |
| replay_trusted_native | 0.89x | 0.75x | 1.61x | 1.71x | 1.64x | 1.54x | 8/10 |
| replay_checked_native | 0.94x | 2.57x | 1.23x | 8.58x | 7.98x | 9.19x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
