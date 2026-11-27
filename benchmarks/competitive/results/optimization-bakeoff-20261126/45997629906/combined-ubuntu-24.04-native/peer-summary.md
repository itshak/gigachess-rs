| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.69x | 1.66x | 1.56x | 2.32x | 1.98x | 30/30 |
| materialize | 0.99x | 2.14x | 1.66x | 1.21x | 2.36x | 1.54x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.22x | 1.61x | 1.27x | 2.43x | 2.29x | 30/30 |
| perft_full_d3 | 1.07x | 1.40x | 1.65x | 1.24x | 2.14x | 2.08x | 30/30 |
| fen_parse | 0.93x | 1.29x | 1.44x | 1.11x | 1.90x | 1.29x | 29/30 |
| fen_write_native_ep | 1.00x | 4.25x | 9.29x | 1.55x | 25.94x | 1.98x | 30/30 |
| snapshot | 1.01x | 2.50x | 1.51x | 4.52x | 1.50x | 1.50x | 30/30 |
| san_parse_all | 1.00x | 1.93x | N/A | 3.95x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.95x | N/A | 2.31x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.89x | 1.42x | 4.23x | 1.36x | 1.92x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.11x | N/A | N/A | 5/6 |
| startpos | 0.99x | 1.25x | 86.24x | 142.52x | 224.36x | 161.36x | 5/5 |
| chess960_materialize | 1.03x | 1.80x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.90x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 35.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 27.92x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.73x | 1.76x | 1.15x | 2.22x | 2.15x | 9/10 |
| corpus_materialize | 1.02x | 1.66x | 1.55x | 1.15x | 2.22x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.37x | 3.02x | 2.36x | 3.04x | 1.70x | 10/10 |
| replay_trusted_native | 1.04x | 0.79x | 1.48x | 1.68x | 2.62x | 3.19x | 9/10 |
| replay_checked_native | 0.98x | 2.51x | 1.15x | 8.76x | 8.79x | 10.94x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
