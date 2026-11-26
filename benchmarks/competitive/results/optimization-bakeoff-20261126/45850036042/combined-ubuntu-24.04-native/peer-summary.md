| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.27x | 1.84x | 1.56x | 2.39x | 1.91x | 30/30 |
| materialize | 1.00x | 2.22x | 1.71x | 1.76x | 2.64x | 1.93x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.75x | 1.55x | 1.18x | 2.30x | 1.87x | 29/30 |
| perft_full_d3 | 1.01x | 1.37x | 1.57x | 1.53x | 1.98x | 2.16x | 30/30 |
| fen_parse | 1.00x | 1.41x | 1.57x | 1.13x | 1.45x | 1.13x | 30/30 |
| fen_write_native_ep | 1.03x | 5.15x | 10.36x | 1.92x | 28.74x | 2.53x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 7.02x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.08x | 1.90x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.87x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.06x | 1.30x | 3.92x | 1.30x | 1.64x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.12x | 92.12x | 166.17x | 197.58x | 162.76x | 5/5 |
| chess960_materialize | 1.09x | 1.80x | 1.75x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.49x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 39.92x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 22.73x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.40x | 2.05x | 1.10x | 2.47x | 2.29x | 9/10 |
| corpus_materialize | 1.00x | 1.38x | 1.29x | 1.06x | 1.69x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.30x | 3.01x | 2.24x | 2.94x | 1.57x | 10/10 |
| replay_trusted_native | 0.99x | 0.88x | 1.57x | 1.54x | 2.14x | 2.51x | 9/10 |
| replay_checked_native | 1.01x | 2.04x | 1.12x | 8.57x | 8.27x | 9.73x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
