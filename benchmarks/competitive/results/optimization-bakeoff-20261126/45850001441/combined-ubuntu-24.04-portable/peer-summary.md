| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 2.91x | 1.73x | 1.41x | 1.76x | 1.48x | 30/30 |
| materialize | 1.01x | 2.22x | 1.76x | 1.81x | 2.54x | 1.75x | 30/30 |
| perft_bulk_d4 | 0.98x | 2.99x | 1.57x | 1.17x | 1.84x | 1.79x | 29/30 |
| perft_full_d3 | 1.22x | 1.68x | 1.96x | 1.70x | 2.39x | 1.97x | 30/30 |
| fen_parse | 1.02x | 1.38x | 1.53x | 1.17x | 1.62x | 1.29x | 30/30 |
| fen_write_native_ep | 1.01x | 3.15x | 10.23x | 1.93x | 28.31x | 2.64x | 30/30 |
| snapshot | 1.00x | 3.08x | 2.38x | 3.16x | 1.22x | 1.00x | 27/30 |
| san_parse_all | 1.00x | 1.82x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 9.36x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.08x | 1.45x | 4.07x | 1.83x | 1.54x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.43x | 56.78x | 102.44x | 132.57x | 108.48x | 5/5 |
| chess960_materialize | 1.03x | 1.70x | 1.64x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.15x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 41.91x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.23x | 32.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.92x | 2.01x | 1.03x | 1.94x | 2.72x | 10/10 |
| corpus_materialize | 1.00x | 1.49x | 1.40x | 1.13x | 1.81x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.94x | 2.94x | 2.22x | 2.81x | 1.55x | 10/10 |
| replay_trusted_native | 0.96x | 0.83x | 1.57x | 1.58x | 2.14x | 1.94x | 9/10 |
| replay_checked_native | 1.00x | 2.06x | 1.10x | 8.42x | 8.36x | 9.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
