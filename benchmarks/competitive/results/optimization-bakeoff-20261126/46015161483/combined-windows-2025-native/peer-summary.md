| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.09x | 5.20x | 1.80x | 1.72x | 2.53x | 2.20x | 30/30 |
| materialize | 1.03x | 2.08x | 1.42x | 1.32x | 2.39x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.11x | 4.19x | 1.71x | 1.42x | 2.40x | 2.13x | 30/30 |
| perft_full_d3 | 1.29x | 1.75x | 1.85x | 1.66x | 2.40x | 2.34x | 30/30 |
| fen_parse | 1.01x | 1.31x | 1.48x | 1.44x | 2.31x | 1.50x | 30/30 |
| fen_write_native_ep | 1.01x | 4.15x | 7.76x | 1.52x | 30.48x | 2.02x | 30/30 |
| snapshot | 1.00x | 1.99x | 1.01x | 4.09x | 1.01x | 1.04x | 28/30 |
| san_parse_all | 1.02x | 2.23x | N/A | 5.58x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 11.23x | N/A | 3.87x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 2.40x | 1.44x | 10.36x | 1.55x | 1.72x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.41x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.25x | 79.72x | 167.61x | 241.04x | 181.95x | 5/5 |
| chess960_materialize | 1.01x | 1.68x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.89x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.11x | 27.42x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.22x | 1.24x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.87x | 19.36x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 5.91x | 1.93x | 1.50x | 2.68x | 2.62x | 10/10 |
| corpus_materialize | 1.00x | 1.58x | 1.26x | 1.17x | 2.02x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.76x | 2.20x | 1.80x | 2.14x | 1.41x | 10/10 |
| replay_trusted_native | 1.03x | 0.87x | 1.66x | 2.14x | 2.68x | 3.10x | 9/10 |
| replay_checked_native | 0.99x | 2.44x | 1.10x | 8.33x | 8.18x | 9.76x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
