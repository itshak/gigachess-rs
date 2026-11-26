| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.38x | 1.67x | 1.80x | 2.18x | 2.17x | 30/30 |
| materialize | 1.00x | 1.95x | 1.56x | 1.34x | 2.16x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.83x | 1.57x | 1.42x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.60x | 1.49x | 1.60x | 1.34x | 1.96x | 1.85x | 30/30 |
| fen_parse | 1.00x | 1.22x | 1.30x | 1.22x | 1.73x | 1.39x | 30/30 |
| fen_write_native_ep | 0.99x | 4.51x | 9.19x | 1.57x | 29.04x | 2.37x | 30/30 |
| snapshot | 0.99x | 1.10x | 1.02x | 1.47x | 1.00x | 0.95x | 19/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.70x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.03x | 1.20x | 1.22x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 77.24x | 128.96x | 174.22x | 141.38x | 5/5 |
| chess960_materialize | 1.00x | 1.72x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.93x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.39x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.38x | 1.68x | 1.34x | 1.98x | 2.24x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.52x | 1.23x | 1.89x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.11x | 2.92x | 2.09x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.62x | 1.62x | 1.98x | 2.22x | 2.25x | 8/10 |
| replay_checked_native | 1.01x | 2.17x | 1.06x | 7.21x | 6.37x | 8.27x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
