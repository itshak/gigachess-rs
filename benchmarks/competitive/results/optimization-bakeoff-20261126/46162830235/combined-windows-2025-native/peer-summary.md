| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 5.01x | 1.69x | 1.60x | 2.54x | 2.09x | 30/30 |
| materialize | 1.00x | 2.09x | 1.33x | 1.16x | 2.44x | 1.62x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.51x | 1.63x | 1.46x | 2.56x | 2.34x | 30/30 |
| perft_full_d3 | 1.10x | 1.58x | 1.68x | 1.41x | 2.23x | 2.29x | 30/30 |
| fen_parse | 1.04x | 1.27x | 1.41x | 1.46x | 1.96x | 1.41x | 30/30 |
| fen_write_native_ep | 0.99x | 3.65x | 7.13x | 1.51x | 31.43x | 2.04x | 30/30 |
| snapshot | 0.99x | 2.51x | 1.51x | 5.40x | 1.49x | 1.49x | 30/30 |
| san_parse_all | 1.00x | 2.01x | N/A | 4.83x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.29x | N/A | 4.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.83x | 1.54x | 1.14x | 7.19x | 1.16x | 2.07x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.00x | 71.19x | 140.13x | 183.65x | 141.19x | 4/5 |
| chess960_materialize | 1.02x | 1.74x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.95x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.86x | 31.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.56x | 1.52x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 27.81x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 6.29x | 1.86x | 1.23x | 2.49x | 2.34x | 9/10 |
| corpus_materialize | 1.01x | 1.67x | 1.29x | 1.06x | 2.29x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.95x | 1.54x | 1.85x | 1.66x | 1.80x | 1.30x | 10/10 |
| replay_trusted_native | 1.01x | 0.73x | 1.44x | 1.73x | 2.52x | 3.08x | 9/10 |
| replay_checked_native | 0.99x | 2.74x | 1.15x | 8.90x | 8.87x | 11.12x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
