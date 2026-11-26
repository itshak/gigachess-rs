| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.73x | 1.55x | 1.35x | 1.83x | 1.30x | 29/30 |
| materialize | 1.17x | 2.15x | 1.74x | 1.78x | 2.66x | 1.91x | 30/30 |
| perft_bulk_d4 | 0.99x | 2.72x | 1.43x | 1.08x | 1.80x | 1.59x | 29/30 |
| perft_full_d3 | 1.00x | 1.47x | 1.73x | 1.56x | 2.11x | 1.75x | 30/30 |
| fen_parse | 0.98x | 1.45x | 1.45x | 1.17x | 1.58x | 1.19x | 30/30 |
| fen_write_native_ep | 0.99x | 3.07x | 9.85x | 1.83x | 26.36x | 2.58x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.26x | 1.25x | 1.00x | 29/30 |
| san_parse_all | 1.08x | 1.89x | N/A | 4.34x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.09x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.98x | 1.35x | 3.95x | 1.21x | 1.43x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.43x | 56.86x | 109.42x | 129.82x | 98.93x | 5/5 |
| chess960_materialize | 1.02x | 1.61x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.98x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 41.37x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.95x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.04x | 17.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.78x | 1.89x | 1.04x | 1.95x | 2.59x | 10/10 |
| corpus_materialize | 1.00x | 1.44x | 1.34x | 1.08x | 1.76x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.94x | 2.88x | 2.17x | 2.81x | 1.54x | 10/10 |
| replay_trusted_native | 1.01x | 0.82x | 1.54x | 1.52x | 2.12x | 1.93x | 9/10 |
| replay_checked_native | 0.99x | 1.95x | 1.12x | 8.53x | 8.26x | 9.56x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
