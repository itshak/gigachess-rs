| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.54x | 1.66x | 1.90x | 2.35x | 2.18x | 30/30 |
| materialize | 1.00x | 1.87x | 1.59x | 1.49x | 2.15x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.14x | 1.78x | 1.52x | 2.38x | 2.35x | 30/30 |
| perft_full_d3 | 1.00x | 1.44x | 1.75x | 1.22x | 1.77x | 1.77x | 30/30 |
| fen_parse | 1.00x | 1.19x | 1.13x | 1.12x | 1.62x | 1.17x | 30/30 |
| fen_write_native_ep | 1.00x | 4.58x | 8.98x | 1.60x | 23.31x | 1.95x | 30/30 |
| snapshot | 1.00x | 1.05x | 0.99x | 1.19x | 1.01x | 0.98x | 18/30 |
| san_parse_all | 0.99x | 2.06x | N/A | 4.88x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.15x | N/A | 2.06x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.06x | 1.58x | 3.01x | 1.03x | 1.04x | 26/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.65x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 63.38x | 88.61x | 127.34x | 92.12x | 5/5 |
| chess960_materialize | 1.00x | 1.66x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.09x | 1.71x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.37x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 30.36x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.39x | 1.88x | 1.42x | 2.31x | 2.54x | 10/10 |
| corpus_materialize | 1.00x | 1.72x | 1.77x | 1.31x | 2.31x | 1.54x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.30x | 3.20x | 2.21x | 3.17x | 1.55x | 10/10 |
| replay_trusted_native | 0.96x | 0.65x | 1.70x | 1.93x | 2.56x | 3.18x | 8/10 |
| replay_checked_native | 1.02x | 2.48x | 1.21x | 8.80x | 7.73x | 10.73x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
