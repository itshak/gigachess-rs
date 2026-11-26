| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.82x | 1.71x | 1.66x | 2.56x | 2.11x | 30/30 |
| materialize | 1.00x | 2.01x | 1.39x | 1.34x | 2.42x | 1.62x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.08x | 1.61x | 1.37x | 2.30x | 2.04x | 30/30 |
| perft_full_d3 | 1.00x | 1.42x | 1.52x | 1.34x | 1.96x | 1.88x | 30/30 |
| fen_parse | 1.02x | 1.29x | 1.38x | 1.41x | 2.23x | 1.47x | 30/30 |
| fen_write_native_ep | 0.99x | 4.28x | 8.21x | 1.62x | 31.41x | 1.99x | 30/30 |
| snapshot | 0.97x | 1.83x | 0.98x | 3.90x | 1.03x | 1.00x | 24/30 |
| san_parse_all | 1.00x | 2.09x | N/A | 5.47x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.06x | N/A | 3.86x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.41x | 1.48x | 9.84x | 1.54x | 1.74x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 86.89x | 162.29x | 236.50x | 174.05x | 5/5 |
| chess960_materialize | 1.02x | 1.63x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.91x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 29.48x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 21.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 5.27x | 1.74x | 1.50x | 2.37x | 2.29x | 10/10 |
| corpus_materialize | 1.01x | 1.61x | 1.28x | 1.21x | 1.98x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.78x | 2.21x | 1.83x | 2.18x | 1.44x | 10/10 |
| replay_trusted_native | 0.92x | 0.78x | 1.49x | 1.94x | 2.36x | 2.71x | 9/10 |
| replay_checked_native | 0.98x | 2.48x | 1.08x | 8.60x | 8.05x | 9.62x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
