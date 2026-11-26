| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 5.15x | 1.89x | 1.79x | 2.59x | 2.18x | 30/30 |
| materialize | 0.94x | 2.04x | 1.66x | 1.02x | 2.22x | 1.35x | 26/30 |
| perft_bulk_d4 | 0.98x | 4.06x | 1.59x | 1.27x | 1.93x | 1.77x | 30/30 |
| perft_full_d3 | 1.12x | 1.66x | 1.91x | 1.48x | 2.12x | 2.27x | 30/30 |
| fen_parse | 1.09x | 1.32x | 1.39x | 1.36x | 1.68x | 1.43x | 30/30 |
| fen_write_native_ep | 1.03x | 4.09x | 8.99x | 1.78x | 27.80x | 2.27x | 30/30 |
| snapshot | 1.01x | 1.68x | 1.15x | 2.54x | 1.14x | 1.07x | 28/30 |
| san_parse_all | 1.08x | 1.80x | N/A | 3.95x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.61x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.14x | 1.17x | 5.61x | 1.14x | 1.35x | 28/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.38x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.18x | 66.96x | 128.58x | 152.92x | 126.69x | 5/5 |
| chess960_materialize | 1.05x | 1.77x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 3.30x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 38.19x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.95x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.14x | 20.13x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.21x | 5.79x | 1.93x | 1.17x | 2.29x | 2.62x | 10/10 |
| corpus_materialize | 1.00x | 1.57x | 1.43x | 1.12x | 1.98x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.81x | 2.48x | 1.78x | 2.35x | 1.36x | 10/10 |
| replay_trusted_native | 1.02x | 0.66x | 1.66x | 1.97x | 1.83x | 1.99x | 8/10 |
| replay_checked_native | 1.09x | 2.27x | 1.14x | 9.06x | 7.31x | 9.01x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
