| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.91x | 4.85x | 1.44x | 1.69x | 2.05x | 1.96x | 29/30 |
| materialize | 1.08x | 2.10x | 1.54x | 1.00x | 2.20x | 1.38x | 27/30 |
| perft_bulk_d4 | 0.82x | 3.48x | 1.29x | 1.15x | 1.63x | 1.66x | 27/30 |
| perft_full_d3 | 1.01x | 1.56x | 1.75x | 1.34x | 1.89x | 1.91x | 30/30 |
| fen_parse | 0.96x | 1.18x | 1.34x | 1.27x | 1.71x | 1.40x | 30/30 |
| fen_write_native_ep | 1.04x | 4.18x | 9.06x | 1.93x | 27.83x | 2.27x | 30/30 |
| snapshot | 0.87x | 1.65x | 0.86x | 1.93x | 0.98x | 0.89x | 20/30 |
| san_parse_all | 0.96x | 1.92x | N/A | 3.94x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.89x | N/A | 2.65x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.04x | 1.09x | 5.31x | 1.00x | 1.23x | 22/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.38x | 77.23x | 167.18x | 224.92x | 165.01x | 5/5 |
| chess960_materialize | 1.02x | 1.73x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.82x | 2.75x | 1.21x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.14x | 43.38x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.12x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 18.76x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.77x | 3.93x | 1.52x | 0.96x | 1.71x | 2.07x | 9/10 |
| corpus_materialize | 1.15x | 1.74x | 1.67x | 1.35x | 2.11x | 1.46x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.54x | 2.46x | 1.86x | 2.66x | 1.22x | 10/10 |
| replay_trusted_native | 1.03x | 0.50x | 1.61x | 1.62x | 1.88x | 1.95x | 8/10 |
| replay_checked_native | 0.93x | 2.06x | 1.00x | 7.35x | 6.55x | 8.73x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
