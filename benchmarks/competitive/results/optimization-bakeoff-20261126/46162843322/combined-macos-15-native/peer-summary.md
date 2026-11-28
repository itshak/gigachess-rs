| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 5.18x | 1.73x | 1.82x | 2.31x | 2.08x | 30/30 |
| materialize | 1.01x | 1.87x | 1.43x | 1.05x | 2.21x | 1.36x | 27/30 |
| perft_bulk_d4 | 1.07x | 4.38x | 1.59x | 1.31x | 2.03x | 2.08x | 30/30 |
| perft_full_d3 | 1.16x | 1.78x | 1.98x | 1.50x | 2.22x | 2.22x | 30/30 |
| fen_parse | 0.98x | 1.23x | 1.33x | 1.35x | 1.64x | 1.38x | 30/30 |
| fen_write_native_ep | 1.05x | 4.22x | 8.88x | 1.78x | 26.85x | 2.24x | 30/30 |
| snapshot | 1.06x | 1.76x | 1.12x | 2.42x | 1.12x | 0.99x | 26/30 |
| san_parse_all | 1.01x | 1.70x | N/A | 3.59x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 12.67x | N/A | 2.82x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.00x | 1.10x | 5.07x | 1.03x | 1.16x | 20/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 67.52x | 139.62x | 159.34x | 133.31x | 5/5 |
| chess960_materialize | 0.95x | 1.69x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 3.12x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 37.84x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.54x | 1.54x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 6.26x | 1.99x | 1.38x | 2.50x | 2.56x | 10/10 |
| corpus_materialize | 1.09x | 1.54x | 1.41x | 1.13x | 1.94x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.89x | 2.50x | 1.82x | 2.38x | 1.34x | 10/10 |
| replay_trusted_native | 1.02x | 0.69x | 1.68x | 1.68x | 1.88x | 1.80x | 8/10 |
| replay_checked_native | 0.94x | 2.37x | 1.07x | 8.22x | 7.07x | 9.23x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
