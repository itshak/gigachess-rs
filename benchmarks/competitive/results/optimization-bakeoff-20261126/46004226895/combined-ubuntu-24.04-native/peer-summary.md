| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.47x | 1.93x | 1.64x | 2.57x | 1.99x | 30/30 |
| materialize | 1.02x | 2.24x | 1.70x | 1.75x | 2.67x | 1.94x | 30/30 |
| perft_bulk_d4 | 1.07x | 4.00x | 1.64x | 1.25x | 2.48x | 1.98x | 29/30 |
| perft_full_d3 | 1.24x | 1.68x | 1.98x | 1.83x | 2.49x | 2.10x | 30/30 |
| fen_parse | 0.92x | 1.31x | 1.28x | 1.08x | 1.38x | 1.06x | 30/30 |
| fen_write_native_ep | 0.97x | 4.84x | 9.70x | 1.84x | 26.84x | 2.48x | 30/30 |
| snapshot | 1.00x | 1.66x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.01x | 1.97x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 8.90x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.10x | 1.39x | 4.04x | 1.33x | 1.73x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.01x | 78.90x | 148.66x | 173.78x | 142.49x | 5/5 |
| chess960_materialize | 0.99x | 1.64x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.53x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 39.41x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.82x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.23x | 6.48x | 2.44x | 1.34x | 2.97x | 2.74x | 10/10 |
| corpus_materialize | 1.03x | 1.42x | 1.31x | 1.08x | 1.73x | 1.19x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.27x | 2.96x | 2.18x | 2.89x | 1.54x | 10/10 |
| replay_trusted_native | 1.04x | 0.92x | 1.68x | 1.61x | 2.24x | 2.60x | 9/10 |
| replay_checked_native | 1.01x | 2.24x | 1.15x | 9.09x | 8.77x | 10.45x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
