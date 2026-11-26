| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 3.72x | 1.57x | 1.38x | 1.83x | 1.60x | 30/30 |
| materialize | 0.96x | 1.92x | 1.38x | 1.46x | 2.28x | 1.52x | 30/30 |
| perft_bulk_d4 | 0.85x | 3.33x | 1.35x | 1.02x | 1.85x | 1.65x | 26/30 |
| perft_full_d3 | 0.96x | 1.59x | 1.73x | 1.58x | 2.23x | 1.83x | 30/30 |
| fen_parse | 0.90x | 1.10x | 1.28x | 1.21x | 1.56x | 1.30x | 30/30 |
| fen_write_native_ep | 0.98x | 4.04x | 7.65x | 1.65x | 29.68x | 2.14x | 30/30 |
| snapshot | 1.71x | 2.07x | 1.13x | 4.88x | 1.12x | 1.13x | 30/30 |
| san_parse_all | 1.01x | 1.76x | N/A | 5.36x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 9.79x | N/A | 4.27x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 1.89x | 1.22x | 9.38x | 1.20x | 2.06x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.37x | 1.15x | 92.06x | 190.90x | 244.18x | 195.64x | 5/5 |
| chess960_materialize | 0.94x | 1.46x | 1.26x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.31x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.59x | 42.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.37x | 26.52x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.66x | 4.08x | 1.61x | 0.82x | 1.83x | 1.78x | 8/10 |
| corpus_materialize | 0.96x | 1.38x | 1.16x | 1.04x | 1.68x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.82x | 2.20x | 1.85x | 2.18x | 1.60x | 10/10 |
| replay_trusted_native | 0.98x | 0.77x | 1.56x | 1.52x | 2.08x | 2.28x | 9/10 |
| replay_checked_native | 0.98x | 2.14x | 1.08x | 8.59x | 8.29x | 9.45x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
