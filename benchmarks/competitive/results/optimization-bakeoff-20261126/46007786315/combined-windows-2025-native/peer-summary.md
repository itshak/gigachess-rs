| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.55x | 1.92x | 1.65x | 2.31x | 2.03x | 30/30 |
| materialize | 1.01x | 2.06x | 1.42x | 1.61x | 2.48x | 1.67x | 30/30 |
| perft_bulk_d4 | 1.07x | 4.08x | 1.63x | 1.26x | 2.25x | 1.96x | 29/30 |
| perft_full_d3 | 1.22x | 1.69x | 1.88x | 1.72x | 2.41x | 1.97x | 30/30 |
| fen_parse | 1.20x | 1.18x | 1.50x | 1.22x | 1.76x | 1.46x | 30/30 |
| fen_write_native_ep | 1.04x | 4.26x | 7.92x | 1.64x | 31.10x | 2.21x | 30/30 |
| snapshot | 1.14x | 2.04x | 1.15x | 4.89x | 1.16x | 1.15x | 30/30 |
| san_parse_all | 1.08x | 1.77x | N/A | 5.36x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 9.59x | N/A | 4.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.10x | 1.37x | 10.59x | 1.33x | 1.67x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.02x | 0.99x | 78.06x | 152.11x | 209.37x | 178.34x | 4/5 |
| chess960_materialize | 1.02x | 1.56x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.48x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 37.72x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.81x | 26.88x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.17x | 5.67x | 2.30x | 1.13x | 2.52x | 2.45x | 10/10 |
| corpus_materialize | 1.00x | 1.44x | 1.21x | 1.08x | 1.76x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.06x | 1.81x | 2.24x | 1.88x | 2.15x | 1.54x | 10/10 |
| replay_trusted_native | 1.04x | 0.85x | 1.57x | 1.66x | 2.26x | 2.56x | 9/10 |
| replay_checked_native | 1.00x | 2.31x | 1.20x | 9.34x | 8.97x | 10.58x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
