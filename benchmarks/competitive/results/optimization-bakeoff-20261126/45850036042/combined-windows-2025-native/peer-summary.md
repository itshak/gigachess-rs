| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.89x | 1.74x | 1.65x | 2.42x | 2.14x | 30/30 |
| materialize | 0.96x | 1.95x | 1.35x | 1.30x | 2.26x | 1.54x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.08x | 1.62x | 1.38x | 2.30x | 2.06x | 30/30 |
| perft_full_d3 | 0.99x | 1.41x | 1.48x | 1.34x | 1.95x | 1.89x | 30/30 |
| fen_parse | 0.85x | 1.17x | 1.30x | 1.22x | 1.93x | 1.31x | 29/30 |
| fen_write_native_ep | 0.99x | 4.13x | 8.31x | 1.54x | 31.82x | 1.96x | 30/30 |
| snapshot | 1.00x | 1.89x | 1.00x | 4.28x | 1.01x | 1.01x | 23/30 |
| san_parse_all | 0.99x | 2.09x | N/A | 5.54x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.55x | N/A | 3.66x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.27x | 1.38x | 9.28x | 1.45x | 1.65x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.71x | 166.25x | 236.50x | 177.78x | 5/5 |
| chess960_materialize | 0.98x | 1.59x | 1.32x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.94x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 27.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 20.59x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.30x | 1.84x | 1.34x | 2.42x | 2.36x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.27x | 1.20x | 1.98x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.74x | 2.18x | 1.78x | 2.13x | 1.39x | 10/10 |
| replay_trusted_native | 1.00x | 0.83x | 1.54x | 2.08x | 2.62x | 3.04x | 9/10 |
| replay_checked_native | 0.99x | 2.26x | 0.99x | 7.89x | 7.62x | 9.01x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
