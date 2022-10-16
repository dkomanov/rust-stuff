#![feature(test)]

extern crate test;

use std::string::{String, ToString};
use std::vec::Vec;

use base64;
use data_encoding;

use base64_jdk;

pub static ENCODED_1: &str = "TWF1";
pub static ENCODED_10: &str = "cmdWcTRWSGtmYUhx";
pub static ENCODED_50: &str = "ZXBueFd1alQwRUZkaGk3bVNXOHhReThoZGRWd1ZQa2llSUxzVG9HakYxYnptbUFmUjlp";
pub static ENCODED_100: &str = "Y2hqMHk1OVd3MXBiZlVFeFpVa1M3dXhVcmlVZVpUWWpBYVZEYmE0dndwbWxtTkRVY2RraHNNeWtVSDZjNmRNUjB0SjNVZDdKcVdhTlhkM2pxZGU3TEw4R3lObWU2T2VUbWxsaWNV";
pub static ENCODED_500: &str = "S0R3R3AwYnAxNGc2UnpueXhzMUVuNkc1MEFaMThPMTg5TWlHaWduMUVrcUFKa05Fem1kTjRFSlAyVUVOOTY2MVpTSDZpZHM3VTdZRFJEZ2JXckZSTHRLa05iRnQ0cHJHclliMTlOZkU5bGlKcnZiR0IxVG5tR0dHckNEVFFqSFo5UVk2VW16UVpkbEJieWNvUEl6YmtuYkF0OVg3c2FKWGZzYlhvMmNSSGdSR0FQQVVxeGU2Tk03RUpBSXN2VnZXRnBlN2hDekFWa0dVS0ZxdmM4cVlIWTNwbEI1RXJoSVdyVXpZekJyU3JOU2UyOEkzbU1GS2dTNVF0Y2Y2WDRCMmpWNFNYVmg1V3V1ZkR3ZFpiZnFnMlFZU2xUZEE1bG1IRXdVMURTMDR2U0FsTmNHRXpWckxPYmNlV0tucUxjRTNZV2x0b1pHbk9JcXRoNXZEaEhjQm00Zm5xN2huclh3NDFXVFB2QjJ1aXR0UmtOaHdocTBlSnhlMHJtb2RyZ2c3SnIzakFwZWNUYTlQRnV6NE9PZDR3TmNMeHBqaTBmNUtDc24yOHJsRUxwN1Z2M0F4Nk45dkVvSnhrY0FlWW9RUjlrdkhZMW9kN3loa2pWaUM0c2xwb3NjWUJjMEVndEdacW5VUWIyRkVobEJDeUhrdXp2MWNl";
pub static ENCODED_1000: &str = "VVBEQ3VGNW0yRlBQUGdaeXh6YVdGWXYxMG5UVWYyM1M1OFU1a0RDeWRYRjRyUDdBN1dFUGdROUt3akc1WDA2VGo2MXQ3Uk5HNFRDWEpYR0JrVm1pSGhack91RmNnVDR4TDRBc0R4d3dtd2N2bHlKU1RqUXZkem43MHd1SXhYZVVBbnI5bUw4TjhyalBnWG5CakRNZWNhZUUxOElWbXcyZ3F1MjFHaDR1N0ZZeHcwSG1SQUtRcFFQWFNGNzVka2Jjb3NVT1AzT1VNVjd6ZFNxYTVhckRDM2pUT3F4N0llaVc1TzNIa2hqblo0dVdJeElVUnZOaVppbDZZNXNCUTJUSkpldldQN1dIaVdOWEprYU92WU9qMjg0Z0NHeXptYlZJN3ZvSnM0eVdLZUdNdWhSYk9Sczg5aEk2dmlVY25ZNmoyVHE5Q2UyYlNLMlZtRkxQaVd0bzdFQ0IzYzBDeFZsbUtaVGtyOHRtMUpUT2hadVltOXlVbkQxdHRzc3ZSemcwZVhFMVdjREZjUmZ1dEZINENHeEdqYU9sUTJxaktkTElaQ0ZBejcxNnJMa096SkdHSDRrRXVza2UyVDhPT3R4Y2xWZ3JHcktNMTh5YXJiS0xBalBYYkJudWlHVTRUUFMxUDkzZHJyRExIbklJTlpDeTExdXJyNHFEMWNhODkxZUlzZ1FFQ3F6d0piY1NiTFlFMTVZNzhqS1VtdmYwQ2RpNW1ZTTM0MlVKU3QyVjZQSHRybkFmZXVvT1o2ZGtKNjlrV014ck10ZldES0VzZ01jNHlLU1hudEpIZHhpbFpYVFNwZk5Lb0hLWk9IbWRKSEFuRVlVWUNnTk9waExiQ2NRNHNKazQ3aWpGTWZRWklPV1Q2Y1NDdHI1VzlHS0dBb1BPb2dPQnczMmFDektJbjR4dmhxREdPZ3ZMcTUwR3Y4QzdKYk1ZODdEMHdtTGNhS2FON1JCbFU1bGRUWDFqdmlGNGZmV21IQ2Y1VkhMRzJXbjZWS0xyMXBqVlFwdExiZlZITGxoTjd1WTR1WkdjSmRuNm1ZTmtmTWhPSWFRWHJ2d0tTaFU1RkZucTlzZHI5bnBCYUNaSU4xNEM0bFpId09KUXoySzl6OHJtSkRZSlQ4R0RLVFNLeDBVNmJaR0ZBM3VzN0dxelBoc0VKUXhpWktIbVRlekpQVTRPUlAwYm5TUzgwd2FaSlR4V3Zic0hUeXEyTjQ0RjZYM0lhcTMwaHdpVGtBTDdoUWRoWVBYUGEybXl2Z0lvNE16UWtwUFBPTms2bzlOM3lCUjA1TVk0VXZPbmdOZ0R2RUZXZ2xaMlpwZmpoQ1lhZnFxdVRv";
pub static ENCODED_10000: &str = "c2hjaG5yNzBYc1Q4a0JSaHp5eXdxY3ZGQjFNaU9GQlJwSFl5NjdDTmFwb1FFa0l0WFRjV3VHUTR4WXR0TlN3Ym5zODNCSTliM1VWRnpNS2NHTVVHYlh3ZVE2MzVYTEprQkIycHBrbVNySW92Q0VvMmpRSFFMNEVKb01FbDd2SnU4eVJoQU1JeVBzVDVLV1N0QzEzaGdJajFpQlFDY0JhNm94YXFZbHZXNlZRMnI3cklsVlZWNnJPT2M5THJRYk94WEVodnNQcEc3aEw5TDJKelFCNW9aeEpCU1lNaTVpa25qZDl1ZjFYWm5JOURqWERNQWxNUTZObjhGVXkwUWwwMFFiTUdJcTFXc21PWkl2Q0p1SkkxbWdXQnl6RlltNml6bTdtVWY4bmVoQjlIcUhaZGdwd08xanJxUk11S2lrdHRqSkJzb2Vxb0NNOE9NVWZhSUM4anZoVVA0ZkR1WW9KQ0lkaWhFUGhzalNqZDhPSGFYaUR3WDQ4eEdNVkpFb1ZSWDd4UjFCRzFmMTVaQkpFQnpGMTFBTU42TDRndlN4bXk4SFRkcVJuZDZQMkhBWDBSb2U4TnIwMzJ6VEhmWWVoYTZhd3ZlNlJlM1VRUmhORUJtOWdPZVJEcExOS0lDWXE5YzRDNzVhR3dyeXBhMHRrMDNZQnZIbzNCc3Z6b1B0dElFVEQ0d2llaTJ3MEZDVnp1cm9aclJpRFZVdjJJaFR4MjRBOFF6eDZFUWlnemE0RWtVSjRoUlYwTmJMblNQNFBMZXBiczlRaUtwUW5Ma3hGWWlObkdka0JSWHlHdzJEa3RMOWxha1RoT2VmUndyWFBGdGZhUkZFWVBkQWRKMlc3WG9vUUFMZW04TmRBd1ZZbVJOZjJzZTM3cjRua1lQTm14cHFnYlhOWW5xbW1LbDVFak05aHFUQTVveUo5UnU3RkI0NXZuT1R0Q1RsbzFtVlV3Smw1OHJLS09WdkFRbXJURlc5OEJsZ0N2SWVEendlZnZvUUh1WUZvOUEyUE5PT1RmZlBqcElNVnhicm5OMUJLRHh4QmxPOFdCNmRMb0VyTTNOT1pjTklmdHQ5NVJBTDRJMWRPaE9XNFdYNkd0MnFNVlRRWnpmTlhwTUFmSTBacXB2aXI4SFpEb0xwVjd4aWFLNlhSb2ZxcmdDWTR3VG5kSW5EY0RnV25NZUdvZzBncE9mZkRZZTBGVHEwQ3JjSHFsTEg1bGxZWm1adzJSVDhlaHIyVG8wVEUwSTR3a0NhRXlUMHVnT3pWUzVrM2J1VzNrb3hwRGpFd3hrUXk5VGNJYk8yVjV0dUFBcUgxRHJoR0VZZ3RNOTV6NGFEQ2xaOVJyVEhDck5xSENPTG8waVprSnhzUFFBakNWd3JVRTU5VEE4WDFoUDlOYjJjZEhOWWs0RHI1aFdSSDVDZU9CQ25yQ1hONHVXTTM3SlM3QzcwOTNOZlZvVU5IMkpDV2pBR1A3ZGdaVnA4Z0M3ckl5VlhRZFJ0YXJ1S1dNMVlXYUl4YTdmV2I5bVlGZUlVUVVsWDBTQkRGNkw3aGpxaVNKOTRVekR0QktqTU5EeTVLa3RFRW5jcHV0V2xXS3dZZUhUbzZKZlpiTmszWjhJYWI2U0dBdE5SYXRMcTRwbXpFajRlVFI5dU9uTTBkUjF1anB0WjBZa0hHYnRuOEp6Zm9pZHhDM3ViNkdNdjhCWlJkaGZwbkNEaW04Y2hOZkw0RXNYVmFsTGFaSWRsVUpXeUs3eVFMYTZmdk5PUWlwTng5TUdjSzVDMnM0dHJjeTVWMHFBaWZsWW83VDVmRzdONGZLOTR0V2IxeE93VFdzbWM4V29uTmxjZEVOMVRhc0ZkRmtnakM0M2hiWjhlN0t4TzZ1SHJzZGl5eTNPR2Q0U2o2c3lsOGZpd2FNakpQT3I1cFJKcktzempaazdMbTNoVTJndWlRRjZlb0JudWdWQlR0OHRiWXFUM3ZidlRjTkdTNFRRN1BpSU1kejU2dGdQZ2ZPelhabWtNd1hjTDJ1Vk5KdFY0bEpsVlBaTjRVWFpzeDYxZTJLUkVRWnpEaW4xV0xUSERYQ1BKd2VRcnpMbU9kM2RCa1pZeUxGVGw5a2M5RWVtYjdOS3NPQjdSRmw1Ymg5ZFZHWktxUGNWZjNRS2F5eXhIS1lsNndkaWRUaE1WWk45ZU5JMVJ1RDhzSGUyZVQwRldTYng1b0oybWpkRnFmSWY1STExSElaZkE3RkJMeFNFaFF0eWtUaG1ZVEZEcTNHZE5hYVBta3RwWDBVNzgydEg4RFNRTW42Y3lTYnNsVlMxY0NQTFFLQjI0anpPVDBqdkQwU0VMTXRxQ1lLeXN0Q3UzeHN3QlRVSVczWnlxMW1BZGR2V205R1J4elk2SGdaUWJCTE5FdlhuQXRDMHpVNXU5NHQyQ29lWjhrdnNJYzN3QnBZNHlyblRmZ0V6cXlRMjJxUDk0c1g1dTNGZU9zTXpIclFCTmJYRzdWVzdVbmxiSG9FR21DMXNtOXo2dk1adDlvYkdKWlMzd0ZzeTJ5cmVZY0NaNklqU2lHMUFPSkVBRzBJR1dqWGhEbFB2OGxWTmVRYmNDRU1VVmJkWVRya0RSVVNlY0xBUWpxOERmYXBrN3R4cDc0c0lqcXo3Y1Y1N0hCMG1TM1lPUEJuNkliWlNva2w3bkE1VG5VdlY1NFBxN0tzUWlQU21ZTU1xa1o1SUEybkxraW5lWWk5aUlPMUJQOHhqcHRLcFljRUg5Z2t3QmhNdHhPaDMyN2p0a0RSOVkwUDd4M1ZZMmxCYzE5VEtsMkEzZUNZQVd1dzhvRGlZSnBTYWlCeUx6a05vaTYwN2VWcE9KSUFnQjJtWDRJb1dRcW9EWlJHOHZaa3J6TGRtbzVNUEozZDRvdzcwMUdJTXdLR3J3RWVrQmRNdjIxeDRqUUViNENUV3hwQ2xFWjdidVM0NE50cGVSY2pSRGFhdFhINzNvSDFsUXNiQXVSWHhNVGhHQTRjYXNnZXJqUG5SU2xhY2dsU3U1ekJSVE1MbDl5OTZ0NDdBMzhqUjJnM0JRcExLTzBhVHpGdGlWTjRGbzBZMmR2TjlHZkllUUJVWXpxTllhV3JDczhkclJEalQ5eFg3RmZsR0xxRGZHMExrRDdXTWE2MEV5TFh5MU1CaDkydUFyeEpGU0U2ZUVTSGE4YkZjWGFaeTZRWUQzSEFWVjRreW5INE56ZUlmUWxWaDdUaEVKM1BDQXVvUEhBTHpNV0djaElVdE9SSG5MeUFMVTZ5Sm92SkMwT1JFVGhVcG9tdlRuY2UxTUp4cHZtSGNUbFVLaEdqeFF2dndPTmZoVjU4dWlET0VqZ0VwODV2dEpibUJzT082amRrc1BlRjE1SEhXRWhzZzFQYW5uRTBwVGNZRkZMQk1XYUNEYWozUWpKWGFoZVQxRHBqb0VXSmtEZGxXSFpwaGtMdmpmS251Vm13VUFZclJlZllHVjM5TUVXNGFhcXRWTmhTa1Mxa2lsdjJqUnlWMDBkN3Y0bkk0ZE1hV2tHYTRxUlg1dk1OWlF0dnF4QVRTQ1NRbXQ2UlQxWnE4dUlwb1pLNktCbE4xTFJ2a3d4THFmSHljZ2t5TnRNZ2JhdUFqM1NjampHZkFUWWI5OE5tMUdab3RwcERKeDN6V0dCMGd4VG5wVm9xM0QyVkpUMDB4ek15blRMQ0NGVzlySVZXNTFPUWZrUlNoSGsxMEZWbWNsODVrUExub2VldWliWHk3ZkN6SkhFcTVCZTFlUGdvdlB3dk5jWTVGUDRhOVBWdFJoeUFQUjVoSnd0eGM4cTdMaDE2WjdPUzRIYkg1SVlSZEx5SDBxb0t0UkxJNHFVMDdPeTRBUGsxaUJBYXdmUFN2djdKQWNpY0k5SGVQQlVEWlRDdWluc0pVdk9OV3ljRmVUSEd5TEJUZEhuQmxmZlA5SHhpb3dFZGtJV3FmS3BiMW9TcldnY0lHUnFicUFhcHc2ZElodlBnaTdNcnpYRWFrQkhhcVRpaTdvTWlsMWxZTktsNEJLdmFQa0NQU3J4c3dtVzVKa2pLU2IyUGFBVVpPMkF3cUxGYVNhdFc5QW9QY0E1M2lFUzFQQXo0R2lrbGlscEhxZFRsZjN2NW9reDJpTHBLakpvcE1PQVJ1ZmNmWnN1SGswVGNnTWZMUEN5Nm11dk1PQkxmU21lRTYzNnBUYVVzdk5WZWMwUkVsZG5MM21FRmV2WGNjZ216UTNDSWNiWldINExxQXg0NDJWclY0M3BqMXBjeDg5ek9Vc2FkNjhlRDhSNXlGWDZpQWtQQ3JLbzhJOU9sNlFXTG5MTnN6d1RmbEUxOXo0SWFaa3p4ckVMMFpJY01UVDJEOTVaRTVEZE4ycmJrNDJ6Q0pSc3RwUWhlMDRIRFVOdm16elpmWTBIN1BSTXJiRnl1aVNxeXhyZ200QmRxR3RTYU9zVjVLRU9EWTNrR0ZVcGVnOXlpM05DYVZkbHJYaG5raEZFZThCVEp4WDhRMHpxSzZoYWVTVTA4aWpDUWJxblV0amdhZW9scDBOZ2J6U2J4OU1CQVIxVzgyVkVoSHhlY3JOWHE1SWhtWlRQVTkyMGRjY3J0NkIzQkpGY0ZBY0JJQlVEa2dYQXNvbG5Fd200NEFEUWl1ejZERDRnRzBRTk1qWFdKOXVoRDdnOTVnd3V2a2I0TGM4cUs4a1VKQ0c1NHNaR0dnd2NoMGlFMkNybjVXdmhSc3dwQWhoTVphUUhXbGNmQW03QTBWYUlQcG1FR05VcDNPb3RMcXNtOEFmVkhvY2JvMG1aQ25ZNE05eUVydm1LQUo1OVlTSDNsR0VNbEpRQUNNRTJ3TVFoTWFETnlpelUzZTFPdGkyM0VVdTlrYUdHUlpZYklFbFpTejdFakw3MXJqQ1RReDRGWk1zZjgwRDdBMDY1Q3dUWWNCd2tzV1dLRldOeFQ2and2V3FMdFFEYk1kSHFRTHNmNUNmWVNBdm56U1F3aXNicWxhYTM0b0NSNk1PWlNOQUI4bnlpelQ4UGlZeENHSFBtVG0zWnY2V1QybVR5UlNIZ2FMbXpUcENHYk1vSWYxNXNtOTBDUzdiTXJxRk1QMnlxUU1oaEd1dExBTkwzVEg3Y1ZUcWxuNzZvTElOSWNlYVp2OG1KUWMzSUFnb0E0SW9naGhBcmk0dXdFMUgzYUIxc04wZUt3dTllVmp0enVHc1BWR2Vjd1NqSTZpYzgzZHp4M3BmdG9wZmkzMTRUOUIzRmV1TkxkSDMwcEJSYTZ5QTVXQzh6Y0lYMHJpdFVZeHZYZ0ZlN2I5dU9zUW1TRGx5aWhiQTg0MGpTMGtlNDFKeWh5cUwzd21POVhRa3p3WllSMWNPZzZGazhvRFoyc0dWUzlDYVQ0UEVhcnVZc3JLTnVBZXllcWI2aXhkOWtORTNLSTl1bFN4VU5IZ2hOWVd5eE40ajZBcnhRaFl1NG9WY2Q4OHRFWW54cUh1MVh1OVZZZnd1Q0ZmYWs5ZFNUT2dRUERxVGRTQ0hNaTdDOEpjd3JCZVBqYkpCYVVacU9ST0pzZ3g4WnlwcU9yeXhqM0tkNlFqdkh3QkFRUTJOaXBwcm9oMUdqNk1aYVR1Z2FrWFB0QUN0ZmxtN1Zib2hlb2gxZldWUHVDZEJoUkpwUzZMbHNnNmxjbzNIT3BsektPeWNOUjZOcEpmaE1TeGVhVEZjN1V2U0IwQUJUM3p3RG14Qnc0dVFvUjk2akJoakZ2Q1dxTWo5SVJ3VFZPeWZCVUpvcWdBSUhtelI5WUc0YnZPRGo4UjJsZmJYZTZoOURUTUgzcHFSMXdEN1FncGRNYkRQZk11U2hPNUg3N3h1bTI4eXk1YWpNSDJKSFdBVTVwcE04eHNBYjlicmNrTzh4ZTNrejJja3Z3OHdRWHRwOVlkSnh0MDA0RUFONlI0eUtiNG1wOXdRYUhCbmswNzBtR2lwV1ZSTHhBMnNWaFlOS2FjaGtPbTl3b1dmNzdHbmlPbTVvQXJHQTl4dnBsTDJjdnIxTFEyVWRkQ0JOMVRpV3daN1BPcVZ6ZGtZMGhHc24xb0lBSWt6Y1NCbXFUZkZsUUhqYU1qdjVUWjdRQ3JiekozeUJOS3dBbmJZNlhrT09Xak5TcmR2cEJHQXRucXVJUmNxbzY1NmM1WDNMa0hDbDdjSXRnR0lXN1lFVGpnclJxRXl2T2ZJOXVsa2pReUxyUDZ4blN1N29aWXJnQnVQamhMa0dqRjkyeXlCY1dLakc1emx3cXMwOEZtVmVRdWg3R0t6QnUwZEVWcTNoVVBURmd1YUlERko2dFdUVzFQSFNaOEJTWWlUNnRscXFoMllldkdJeGhGdkRIdG5zNjVsMFdJdVJ6Ym9kSFNjQXNNcWdLM3BMbnpyUEVGTlBpRzczM3BTV1FBY3VIeTFaSlI2dXNSZThBSUtoQXV2YjZraHNIa2FmeE95dUJ2ZldsTzVyZlBYYTlReDJWandmRzlGV01wQmE2d2ZmMW5kTjloa3B6MTBRTnZHN3Fwa1BKRmpRNFFXYk9hRlNaZFVyQ2xXVTRia0FBczFGZXdsVTVEclVkUkRGdXFWck96R05ua1ZNUXFsejNnSkYxNEVmY3VadjRHYnVPZGdkd2x4ekF5N2R2amw1dEFWY3N1ZjZyVFBxZDdCbElBWTZKa0JTbTNSRzhFemRMNlNaN1Nqb0pPUmFxNDd0MEJlcGJ4QUw4cnhncHJSemlHdDdNRFdja0hQWHBZNXNKdU5qWWNwT0pVM3lXQlZoUWRDT1d6Z1MyNlBvb1gwSDBmNERublFaU2F0QjY2dW5tS0F0NWtabXJ5UGRWUEJwZkV6NGNyQUdrcEdpZ3hBcnpkOHE2YlY1a0pySDhBUkttak43MXFMMVUxUEZPWUw3MG02ZXdaeFNxdHlqVVpKU1lKRGxZdWRQcTc1emJ2RGZPa1BMc0traDUyY0h2ZGE5YkhOTlJVU2RWZGltbDV3WHhvSUNCQU9oMWRSTG5reG5CRENLRTc2RHFXTDRyVzF0V0xySjlXSTN6VWVhWFhlRzhSVjd1cmRBSDlCZ2pYWERHMTFJcnc5akxOUXloQUhlSkpHM2RvSlY3U2V3U1o2TllVejBYVkFxUlphSlVGM2E4M0lwbnBGSVdvek92QXNyelJoVnBwUmEycGtzdFRrYkVaNDJlWUt1a2JoWGl1RnpBeFduZk1JSVd3TjZ5bDlETUtDT1NVSmJ1bDJXZkZndlhhRG5xSmpnRXhtZ0YzUU5YVzNleWVMamxtbnV0NDUwelIwUWl6ajFvbHZlRFRwOXhZT2dlcWs0N216OHQ4Y3NmM3lod2NmbGxMcG5LVUp3cjluMEV3c283UGMzWFBrWUZHZ3BkWXF5cU9GR3dPY1doTDlMWWNMVkc4aFdVSGx0bDJqU3M4dTZkaWlHVkwyZUI1NXZRbnB0VG85aFNRblBjNnJ2OHVDbjVtbzB0R3ZpdjVJZ1VKU2lYZXdQZWw0bk1WNG1IRVR0RlF4cEsybHV2Qnoxak9JNGpHNVNlOVp5UzcydVFaQk9xTENkZ2JqR09QeXBYb0tEeEU3dmh1RUd6NGdMWDJFdzdrbEdzRklTZ043dEM5R1ZDZzlHc0JQZkM5c1dpZEluYjdlSTIwM0dXNVJRdHZPMmxWWk9yR3AzNmxXQkVtZGd2cGJ4ZTlWczlSTDNCZGhPZk85cDJYaUt4T3BwTklRNWp6VDRGbEh3d0phN29GRXRUaXZtZWZobFdSVWNxOWZGY0RyZVhPMkcwUlhRVFhvS0M1Y3AzNGE1V0ZZNThGWms5UkEwaHBGVzVxOG5TSDVka3JhM0JuSm1sdTI5UjVFc3Npb2c5MjhHUzRyZUhxY2RVQ1BqMHVFSFFBQXZUY1VLUUUyQ2p0dEpHc2ZRUnZRWkI2c0E0bnRGM3lPcXRrU2tsQ2w5QmJyQmtmdlR5VXBISEZ5TlBOTG1aRjFqWDVBMkNoM055YXd4dGhFMkYwRWdVUXdoQjdPaEM2anRKV1lvbnRiVlFwTVB6b1A5Snhvakx3SlZueHZnVVpZTHp6czFhcnNVVUc0OHB2MDF5SGVZa2JIYnZybVl1bW9HVmpwc3lyTFVZRG5LTTNBVXNWeUg0dHBpZmExaGlBVzBGRXpYNUVFbGlyd05KeU1IV1d3bUFISjZteWQyUGlmdHVHdUdKaEwwVjF0N0p2MHRlR0pSODZnWUxHSVpaeUdPNnFsWWpVVWkxcnoxa1J4WHFyall6UEg3d0M2TzZ3bnZqTEVLYVY4ZzRmTXlIOEVnNm84cHFNR3VXcTc1cUJiRkpCUlVBZk5yQXE1bzM4UGRTWE1kUXJTaDlocThpSURGTzBrTzk1RkVWMEdmWEVFaEt5RDVtQjBmbnlabkdzZGpkRmlRVVZINlh4WEEySm9OSGdmSEhQUG5EVFlzWHpJRXFNMHg5VWNOZ0FWNjgzSENOa0EyMmZMWUxhRWNVanJrOUVIRHExRnpqb1FIR2x5V2lXbHFQdGo5N2lIRHVkTXNvUEpEMmVpdEdYdVVWN2p3eWlsTkRzd0ZEQlZ1TDZlamZQbTloSnVoZGQzMm1kR3NybU5FZlhzbjNMYmQ2VW0yY3RNYUtQZzBJektKb1hHVlVQeURBT0tzZHZ6cTJReUdRNWtxeGo0V1JidVdlUGN4N0dHbnplTkwzNlBHS25Ic2hlbGhIZmpteFAwejFsWXhBN3pBWTRkWE9VS3AzaEFmRDdFMWQ3cWEwTFFOQmpoYTFxbFdxYTNyU2hRR3dRVXZFcUZlSncwOWFIRFNqMUFiSzJPaDJ4T0d2TXZHcUhMeVNMQ0NlZDFzaDNVcml6VHFKVXdKcGx2cEhCanI3M2dsb3lySlIzTHN0ckZHb29aREhHcktRa0E0RFlYZkF5VzRvUVpwekVaRzNObWxpd2pYNGdZZ1ZJclFyWWtNWTJ0Wm9TTThyMlhyWkhYYW5hV0pBZXZmNUhrUEg4TnNOU1U1dTJDVXM3WDQzbm10cVZQS1FCY1ZkTm0weHF5bFE1OWVjQTh2dlZ2bzE5eXdzbG1uN0cyTHVGMWxiYzJaQmRqM0g0UWlLbzZrVE9aVlV4WGdQbDZOVFM3eG1EbUFxcjUzRGp5RlF2cnJLOGljcjNwZmZjcjBBRTdiM3RRNlNjUXRiRVFGQTBycWNWRkpUQmxybk1sY003TVllSmVzTXViazZEWmNzVW14ZjZ1eFpuME5WbXJDcTZQTnczcVBTajJUSHhOcHQ4UjUzMlFTNFhGMGFlVUVqbXpKUzVDUFlzOHZVNm1JUDBUc2NGMURYeHMyRzI0cDRNRzNPNmpWR2t2NzJTUnA5Z0l6d2R4b0hXUWlTU1VhOUFnV3ZaRXhMdWtVUmNhR1JCTnU4ZTZLaXJ2dHZaREh2OGdkc3U1Z09lRUYydzRJM1FwU0VSemYxd09vZWtHYWtCWHRWNzB5NVFobml2dk1BNU9uVXRZT2RiRVR5UHhaUTRJNTZmWmNnREFGM0pqUU81aGhXcU5ZSEtOQXZxSzJsRm1qNXNzSTl1S3FXODJjcWpwZFNFdHRxZGpRSXhVeHJaaFl5TG04WkhBZ3l6bXZJNUpVbGd3SGlXTTV6aFRJVldtMFlJRndyRzZPWnpzNk9GVkQxaTZZRUVZMTA5Slk1WDBZNVZiZUpzeGtYd1h0bm1zZllZV0s3ZUFrclZFNFVqMTJuSFZrRDhtYmRHeG1aekpqeWFtQkVpWGthVUtRWHVRcEVIMGN1djVsTTJmck5jOGVST29KaTZjb3VtVlhnc1Y3eEJMMFNkM3J5MkdmejBxU3lFMzJpazcyUHhGbTJ1UjJFdHNNZ3lITTZjcEVmd3paWFFseTB5OWFmTVRwUGJUZG1HVUlYSk5oR2RhSGRvUXF4ekk2SFlYVk1McVY5NTV5d3VTem5Md04xS1RZY2N3TmJFd0hrbE01S2x2V3RQSEJSVFdkaklyQmxKc0lXRkN2T1dNMTF6NkhpZTVzaTdGM21HZWxPTGJTZEU2Zm9NdkV5NGFYNjk5ZDh5VVhlVVREUEFxdDFqbVBjaUNaYnJmbk0zd25NT2xVV3c4VlRSaGg5N2llTnMyYnE0VFF5dFJRMlZMRDc2bFlNOVZIaFVFbGlYN1V6U3RKUjdUQzNITFVKVExJd281TWlMUmY4RHlVS1BMd0hJR0Eyc05KbGJteXdKa2VXT1gxQnlxMk1qeFFObHBGZTdSZ0dHMjFrcmx6Uno5TUdXRTIxTlVwbWxZN1VCcFZXUEEwZm5RaTRRaUFOWWVHYTRMWTdxOFl1ZEFjOXU5VzcwMldjSHBJZnBZZUhjZnl6b1M2c0lnQzMyYnA5TldzYmx5bnNGOXZ6c0NvcHczZ1ZPc0ZSaEFweFBrM1BrMmZacDFScnNJYkhQNWRMUTRnNjRVNlQ1RHRFTVM1ZTdWbDFJSHZpOVQ5YlRadXRVMGFnMlpjQ2hkUWVqOGNUdFZZakgxZXNsUTVJRnhxSFJHTmtrZVdIR1FCM1ZzTTRpYzMzQjg1M1JBa3JEa2t5Q0FIZUxKWHlheGdMSkJHc0pKZkRzaW9yVHRpVEM4MHdkYUJpRHBsakpTc3p6Q1FGTmxZdWhjbE03dnVNWncxOVV6MXR4NFZQdWRJV3NuSkp6ZzlRSXJUTzhiRzVJem45MkFqMEVuUjIzeEFtYnBZMzI0VEFoS1BLN0xIME9SREpGa0MzcXIybG9ZTHY0THpOaFgyWG9kVHlzVHVydnJqRk1BYWY3eUdaWUNVQzMySUdKenJRRE9Gb25MRElUVU9iWFB3YkczYVZpZ0xwTm00elRqSXFtRXFTOEpnT01rakhhVXp0dzNaNjRNdzNIMUh0b3hMRDZkOUFNbXpOMDA1MWM2NU45ckdjazRacDdXNWJnZktBc1Rwek9GaUlON0NEOGpsTjBKYVdGN2dQUDJOcGdvREVFeWtocU1KclFrYUpvUjZadXdFZ3J2aGEyOFA4ekFNbTB5N0VDMzA2T05ZT3VFY3hmcmxxcU9lNmQzOG1XQWpibzd3SzNlQ09WU0RDMU1SNDFVQWVjQWs3bGZvbDVZa1MyVk9URXpNQlIzYlJ0ZVROQklPSUZYY1ZlUkV4RTRaZnNSVmt4WjAycWZDY0hIQ29TUXNQMzZ5d3VtRUd6dmlYN01JUndsWmp1Y1A1dk1MVENXVGxlU05ad3lmMVM2Wm4zOXlTTndsQkRLVlZmOTFicXpXMnFEWnM3Z3FPTXEybHhMNkI1b2paNlVSZldlOFMyc2dWREpGMHU3WXo5WXZoc3F0U1hUZFplMzVmR0J4djhEWHZiVzF5Vm5DWUk3WHFzZVF6bm42VUQya1RLWnpYWVh1NG9Hc3BXYmpoRG1HZWt2QW5SZHViTkNNcWpaME9ZQkhYZnU0UUl5bXQ4djdjOURReWdFdFJaTGVhTGNibDZ6aVFRZkQ3MTRNTWcySUpwbmxpMkxwMUNwa0RvU2M1aUFvMXZwckJrdXNNZjhnQnFLb1pQaVlYQlhadHk5OTlvMUpTVjdjMXk1ak4yVVhUZG1jcG5MNFk4MXN5QURZSXV2ak1xS0dPSGpLTGN1dmhsSW5OSXczZ0Y1MFFoWXl6NWxYWDJIWnpZWFRNZ0RNUjhpb1plQkIzRlpTRFQ1dlJqblNiMTRST3p3NzVGU1ByU2ZKTkpKbTdQa3ZlWTU2UUo3SGE3VkdJNTVzYWd3RkZBWXB4dk5Xd2ZkNkJ1c2JQQU9uWW1xcXM3U1p6SkpBQ3RHb0RHakhQaTVJWVFZUFRzRDlBanllbm5vYVh5VklLeElaZW1iQVk0dWVVZHpveFFyVVI0Tkk4NzRNWkRVOEN2MGdtSlRYZ0hMeFJ4bFhwV1E2eG4zZnRxUVVmWTEzZm9BdjdpWko0N3pPMGdmbVVTVlF2VjNEam5CSjBSQzl1UVpiRFpXSzFxNmZKTmJsVWVMb0tNbGp0aUtrYzR6WGFPMDI4V2lwZVNyOU81YkF0Z09YZXdCUllhUm92RWJDaEtLaXQ4MTI3d2RPRnUySEJHSHJPNGNXUGJKSU5ibVFVb3N0V0NLeWJobWZqSWczeGRQZlZ2V3hjTDdGOVR3YUlBczVwSTVNTnFUdVEzYVlIb2xONUJoczZHRFMxZk5jSU1EaDhLM1kxbEpVUGcyaGlGUjJla283cEZNbTl2d2s3ZVFkSndrbXFLcWNxV3dCdVlPdWlIekNuenJpUmJJM1RIbDE4cEtHa2NmZkRSQ1U5SHJFTFBzdEF0ck1MZEhCam9hNmxHOWt3Qm5mcjMwZUcwMlRVZ3JtZTJhMVUzb25FbDB0VFJSMDh1TEF1Zm5oTGhMY1ZFYlo0aXF6cVpRVGpxVk5tbTlVd0dQOVdrcWxGbDMyVzR2YWV5UTNKb1c1bEwxQVlBWDFYRm5EVnNKQVNwY1cxTWNkN2V3RFBhdGhnMDR4Y1M0cTNpQnFQWEUwQkoydDEyNk5TWjh1dW51MzVQQ0RNTzJmM1BneWlQdGhQTDNJa2JpcGg2dXFnWFdIVm5WbmxDWGRSUGlvZ3hBQkNaWmtqQ3ZqSnIweHNUUFVuNFhyWFNDdXUyN1pRS3l1UGJtM1JoM2FXejI0ZWtwYXFOQVJnWnhycmVpaUswellGN05rUlU2bUxJS3dpQ1dpUjkwWTBjMm15VEFEb2pXUUJPaHQyajZaV0NBY0tpRUU1T1ozOGhuVUdJOHpjV0pxdzRrZkVmSmozUmtUd0dVNGdYQmxleVZEbWhwZUdudzN0a0pjTVZFWHROZ01keXRkQmJkSFBhVVRoaWxQeFljdEk3eXBJMUcxNFkyYW5uakg3bUpiSHZrOEN2MHl4YzRUdTF5OGtySmVpWDJ2bUg2SjRXTG53UktpYlJoeXlBaEZvakloY1d5VUFyRVgwMTRJOXFHb3Zob2RKenJNUTJXN0pMT2pETmlaTFhLcmYycjJETFBQQU1sajh1VnpaVWs3d2E1elp3VXBWZGhDendrREREOHZMOFBrS0tLVVFoNjd4Rk0yZzlVRk1IRGxQenB6RzhTU29Wd0F2SE1JZmxUbFpQNkZtT1FUQW51QXoxMG96QnFmZm9FejF5Nm84TGVsalJrbFo2Z2trUTF5YjlJMkRKQkF5eUpwNXNMdGx3TmRxNThEUUNGTXRudFVwTmxqSU8wWFJONmlMMFhNWWU2NXQ1bURnOEZKQkw4M0Z4WkxkSnhieGJubkdLUG9FWFpQUFpwamJvVVQyUlpkZW5iVXJsRW9xQm5BMTAyZTVyaFh5MEtLMDdqdnJDaDZRWlZaRFNlYzBQd3E3Zjg4SE5ibGRtb3h1OE0zc0xqWDVkdlZZUFFtSnpp";

pub struct TestData {
    pub encoded: String,
    pub size: usize,
}

impl TestData {
    pub fn new(encoded: &str, size: usize) -> TestData {
        TestData {
            encoded: encoded.to_string(),
            size,
        }
    }

    pub fn get_payload(self) -> Vec<u8> {
        base64_decode_config(&self.encoded)
    }
}

pub fn get_all_test_data() -> Vec<TestData> {
    vec![
        TestData::new(ENCODED_1, 1),
        TestData::new(ENCODED_10, 10),
        TestData::new(ENCODED_50, 50),
        TestData::new(ENCODED_100, 100),
        TestData::new(ENCODED_500, 500),
        TestData::new(ENCODED_1000, 1000),
        TestData::new(ENCODED_10000, 10000),
    ]
}

#[inline]
pub fn base64_decode_config(s: &String) -> Vec<u8> {
    base64::decode_config(s, base64::STANDARD_NO_PAD).unwrap()
}

#[inline]
pub fn base64_decode_config_buf_no_prealloc(s: &String) -> Vec<u8> {
    let mut buffer = Vec::<u8>::new();
    base64::decode_config_buf(s, base64::STANDARD_NO_PAD, &mut buffer).map(|_| buffer).unwrap()
}

#[inline]
pub fn base64_decode_config_buf_excessive_alloc(s: &String) -> Vec<u8> {
    let mut buffer = Vec::<u8>::with_capacity(s.len() * 4 / 3);
    base64::decode_config_buf(s, base64::STANDARD_NO_PAD, &mut buffer).map(|_| buffer).unwrap()
}

#[inline]
pub fn base64_decode_config_slice(s: &String) -> Vec<u8> {
    let mut buffer = Vec::<u8>::with_capacity((s.len() + 3) * 3 / 4);
    unsafe {
        let mut sl = std::slice::from_raw_parts_mut(buffer.as_mut_ptr(), buffer.capacity());
        let size = base64::decode_config_slice(s, base64::STANDARD_NO_PAD, &mut sl).unwrap();
        buffer.set_len(size);
    }
    buffer
}

#[inline]
pub fn base64_decode_config_slice_memset(s: &String) -> Vec<u8> {
    let mut buffer = vec![0; (s.len() + 3) * 3 / 4];
    let size = base64::decode_config_slice(s, base64::STANDARD_NO_PAD, &mut buffer).unwrap();
    buffer.truncate(size);
    buffer
}

#[inline]
pub fn crypto2_decode_config(s: &String) -> Vec<u8> {
    crypto2::encoding::base64::decode_with_config(s, crypto2::encoding::base64::DEFAULT_CONFIG).unwrap()
}

#[inline]
pub fn jdk_decode(s: &String) -> Vec<u8> {
    base64_jdk::decode(s, base64_jdk::STANDARD_NO_PAD).unwrap()
}

#[inline]
pub fn base64_encode_config(s: &Vec<u8>) -> String {
    base64::encode_config(s, base64::STANDARD_NO_PAD)
}

#[inline]
pub fn crypto2_encode_config(s: &Vec<u8>) -> String {
    crypto2::encoding::base64::encode_with_config(s, crypto2::encoding::base64::DEFAULT_CONFIG)
}

#[inline]
pub fn jdk_encode(s: &Vec<u8>) -> Vec<u8> {
    base64_jdk::encode(s, base64_jdk::STANDARD_NO_PAD)
}

#[inline]
pub fn jdk_encode_measter(s: &Vec<u8>) -> Vec<u8> {
    base64_jdk::encode_measter(s, base64_jdk::STANDARD_NO_PAD)
}

#[inline]
pub fn data_encoding_encode(s: &Vec<u8>) -> String {
    data_encoding::BASE64_NOPAD.encode(s)
}

#[inline]
pub fn data_encoding_decode(s: &[u8]) -> Vec<u8> {
    data_encoding::BASE64_NOPAD.decode(s).unwrap()
}

pub fn base64simd_encode_to_vec(s: &Vec<u8>) -> Vec<u8> {
    base64_simd::STANDARD_NO_PAD.encode_type(s)
}

#[inline]
pub fn base64simd_encode_to_string(s: &Vec<u8>) -> String {
    base64_simd::STANDARD_NO_PAD.encode_type(s)
}

#[inline]
pub fn base64simd_decode(s: &String) -> Vec<u8> {
    base64_simd::STANDARD_NO_PAD.decode_type(s.as_bytes()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_successfully() {
        for td in get_all_test_data() {
            let input = td.encoded;
            let payload = base64_decode_config(&input);
            assert_eq!(base64_encode_config(&payload), input);
            assert_eq!(crypto2_encode_config(&payload), input);
            assert_eq!(String::from_utf8(jdk_encode(&payload)).unwrap(), input);
            assert_eq!(String::from_utf8(jdk_encode_measter(&payload)).unwrap(), input);
            assert_eq!(data_encoding_encode(&payload), input);
            assert_eq!(base64simd_encode_to_string(&payload), input);
        }
    }

    #[test]
    fn decode_successfully() {
        for td in get_all_test_data() {
            let input = td.encoded;
            assert_eq!(base64_decode_config(&input), base64_decode_config_slice(&input));
            assert_eq!(base64_decode_config(&input), crypto2_decode_config(&input));
            assert_eq!(base64_decode_config(&input), jdk_decode(&input));
            assert_eq!(base64_decode_config(&input), data_encoding_decode(&input.as_bytes()));
            assert_eq!(base64_decode_config(&input), base64simd_decode(&input));
        }
    }
}
