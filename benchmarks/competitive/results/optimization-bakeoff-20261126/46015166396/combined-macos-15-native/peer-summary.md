| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 5.07x | 1.82x | 1.87x | 2.20x | 2.01x | 30/30 |
| materialize | 1.15x | 1.78x | 1.50x | 1.10x | 2.17x | 1.37x | 30/30 |
| perft_bulk_d4 | 1.16x | 4.46x | 1.77x | 1.50x | 2.22x | 2.13x | 30/30 |
| perft_full_d3 | 1.19x | 1.82x | 1.93x | 1.53x | 2.13x | 2.32x | 30/30 |
| fen_parse | 0.91x | 1.20x | 1.38x | 1.29x | 1.59x | 1.35x | 29/30 |
| fen_write_native_ep | 0.88x | 4.23x | 8.63x | 1.72x | 26.21x | 2.14x | 30/30 |
| snapshot | 1.06x | 1.31x | 0.99x | 2.03x | 0.83x | 0.97x | 17/30 |
| san_parse_all | 0.94x | 1.85x | N/A | 3.67x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 13.44x | N/A | 2.89x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.21x | 1.31x | 6.16x | 1.24x | 1.43x | 29/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.95x | 1.19x | 85.67x | 166.00x | 208.68x | 173.41x | 5/5 |
| chess960_materialize | 1.19x | 1.85x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.32x | 1.63x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 42.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.59x | 1.64x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.92x | 20.03x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.24x | 6.52x | 2.30x | 1.31x | 2.63x | 2.75x | 10/10 |
| corpus_materialize | 0.78x | 1.34x | 1.06x | 0.98x | 1.79x | 1.02x | 7/10 |
| corpus_uci_sorted | 1.12x | 1.85x | 2.63x | 1.82x | 2.45x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.64x | 1.69x | 1.66x | 1.92x | 1.60x | 8/10 |
| replay_checked_native | 1.12x | 2.67x | 1.41x | 10.53x | 9.02x | 11.11x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
