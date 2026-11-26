| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 5.02x | 1.71x | 1.60x | 2.42x | 2.10x | 30/30 |
| materialize | 1.00x | 2.07x | 1.34x | 1.16x | 2.35x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.19x | 1.48x | 1.40x | 2.32x | 2.24x | 30/30 |
| perft_full_d3 | 1.06x | 1.42x | 1.51x | 1.35x | 2.02x | 2.07x | 30/30 |
| fen_parse | 0.91x | 1.21x | 1.41x | 1.41x | 1.92x | 1.37x | 30/30 |
| fen_write_native_ep | 1.01x | 3.73x | 7.41x | 1.54x | 31.73x | 2.07x | 30/30 |
| snapshot | 0.99x | 2.51x | 1.50x | 5.44x | 1.50x | 1.49x | 30/30 |
| san_parse_all | 1.01x | 2.02x | N/A | 4.71x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.27x | N/A | 4.03x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.90x | 1.40x | 9.08x | 1.43x | 1.95x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.01x | 69.18x | 135.23x | 181.41x | 135.98x | 5/5 |
| chess960_materialize | 1.00x | 1.74x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.91x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 36.65x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.22x | 1.15x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 27.38x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.91x | 1.76x | 1.18x | 2.31x | 2.23x | 9/10 |
| corpus_materialize | 1.03x | 1.66x | 1.24x | 1.13x | 2.22x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.61x | 1.96x | 1.80x | 1.92x | 1.39x | 10/10 |
| replay_trusted_native | 0.99x | 0.72x | 1.42x | 1.72x | 2.48x | 3.00x | 8/10 |
| replay_checked_native | 1.01x | 2.57x | 1.21x | 9.20x | 8.86x | 11.41x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
