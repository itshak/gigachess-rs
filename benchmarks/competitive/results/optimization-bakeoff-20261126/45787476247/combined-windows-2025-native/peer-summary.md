| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.34x | 1.57x | 1.52x | 2.39x | 1.90x | 30/30 |
| materialize | 1.05x | 1.90x | 1.27x | 1.07x | 2.16x | 1.39x | 28/30 |
| perft_bulk_d4 | 1.06x | 3.80x | 1.58x | 1.35x | 2.35x | 2.12x | 30/30 |
| perft_full_d3 | 0.99x | 1.27x | 1.34x | 1.04x | 1.80x | 1.59x | 28/30 |
| fen_parse | 1.08x | 1.32x | 1.50x | 1.35x | 2.01x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.00x | 8.01x | 1.59x | 30.68x | 2.00x | 30/30 |
| snapshot | 0.99x | 1.73x | 1.22x | 3.22x | 1.22x | 1.21x | 30/30 |
| san_parse_all | 1.08x | 1.94x | N/A | 4.74x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.39x | N/A | 3.36x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.50x | 1.03x | 6.15x | 1.14x | 1.35x | 27/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 0.88x | 0.88x | 67.22x | 117.02x | 172.10x | 133.73x | 4/5 |
| chess960_materialize | 1.00x | 1.54x | 1.25x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.56x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.38x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 17.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.46x | 1.81x | 1.31x | 2.39x | 2.17x | 10/10 |
| corpus_materialize | 1.00x | 1.74x | 1.41x | 1.17x | 2.84x | 1.46x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.86x | 2.36x | 1.94x | 2.23x | 1.48x | 10/10 |
| replay_trusted_native | 0.97x | 0.71x | 1.25x | 1.55x | 2.41x | 3.14x | 8/10 |
| replay_checked_native | 1.00x | 2.17x | 0.96x | 5.43x | 6.70x | 7.68x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
