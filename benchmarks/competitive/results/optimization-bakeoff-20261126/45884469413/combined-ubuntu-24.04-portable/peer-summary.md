| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 2.88x | 1.66x | 1.45x | 1.93x | 1.43x | 30/30 |
| materialize | 1.13x | 2.41x | 1.89x | 1.90x | 2.88x | 2.05x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.04x | 1.62x | 1.19x | 2.00x | 1.82x | 29/30 |
| perft_full_d3 | 1.00x | 1.36x | 1.61x | 1.42x | 1.90x | 1.59x | 30/30 |
| fen_parse | 1.12x | 1.58x | 1.49x | 1.22x | 1.54x | 1.23x | 30/30 |
| fen_write_native_ep | 1.01x | 2.98x | 10.21x | 1.86x | 29.12x | 2.60x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.24x | 3.24x | 1.25x | 1.00x | 25/30 |
| san_parse_all | 1.01x | 1.94x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.29x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.22x | 1.46x | 4.25x | 1.36x | 1.58x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.36x | 56.95x | 109.12x | 126.48x | 108.68x | 5/5 |
| chess960_materialize | 1.08x | 1.84x | 1.82x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.13x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 37.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.07x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 19.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.16x | 2.09x | 1.14x | 2.12x | 2.91x | 10/10 |
| corpus_materialize | 1.00x | 1.51x | 1.42x | 1.15x | 1.83x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.96x | 3.00x | 2.21x | 2.97x | 1.55x | 10/10 |
| replay_trusted_native | 1.00x | 0.92x | 1.68x | 1.61x | 2.25x | 2.06x | 9/10 |
| replay_checked_native | 1.00x | 2.14x | 1.16x | 8.98x | 8.86x | 9.93x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
