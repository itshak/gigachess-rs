| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 3.70x | 1.61x | 1.38x | 2.13x | 1.68x | 30/30 |
| materialize | 1.03x | 1.97x | 1.53x | 1.62x | 2.27x | 1.67x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.26x | 1.34x | 1.00x | 1.94x | 1.61x | 27/30 |
| perft_full_d3 | 1.00x | 1.50x | 1.80x | 1.65x | 2.10x | 1.84x | 30/30 |
| fen_parse | 0.98x | 1.36x | 1.47x | 1.10x | 1.54x | 1.22x | 29/30 |
| fen_write_native_ep | 1.08x | 4.85x | 9.70x | 2.11x | 28.55x | 2.67x | 30/30 |
| snapshot | 0.98x | 1.95x | 0.97x | 11.65x | 0.98x | 1.26x | 21/30 |
| san_parse_all | 1.09x | 1.88x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.39x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.92x | 1.18x | 3.65x | 1.20x | 1.51x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.10x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.22x | 112.14x | 169.72x | 217.77x | 171.50x | 5/5 |
| chess960_materialize | 1.02x | 1.47x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.21x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 41.05x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.07x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 21.46x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.15x | 1.59x | 0.84x | 1.97x | 1.77x | 8/10 |
| corpus_materialize | 0.98x | 1.37x | 1.26x | 1.01x | 1.61x | 1.11x | 10/10 |
| corpus_uci_sorted | 0.92x | 2.17x | 2.83x | 2.15x | 2.68x | 1.43x | 10/10 |
| replay_trusted_native | 0.98x | 0.79x | 1.41x | 1.37x | 1.93x | 2.31x | 8/10 |
| replay_checked_native | 1.04x | 2.17x | 1.14x | 8.92x | 9.01x | 10.30x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
