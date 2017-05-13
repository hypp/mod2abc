
; song data
song_instrument_list:
	defw song_instrument_0
	defw song_instrument_1
	defw song_instrument_2
	defw song_instrument_3
	defw song_instrument_4
	

; an instrument is one command per frame
; until a 0 command is found
; remember to turn instrument off with AUDIO_MIXER_SILENCE

; kick
song_instrument_0:
	defb AUDIO_ENVELOPE_PASSTHROUGH+AUDIO_MIXER_NOISE+AUDIO_ON
	defb AUDIO_ENVELOPE_PASSTHROUGH+AUDIO_MIXER_SILENCE+AUDIO_ON
	defb AUDIO_ENVELOPE_PASSTHROUGH+AUDIO_MIXER_NOISE+AUDIO_ON
	defb AUDIO_ENVELOPE_VCO+AUDIO_MIXER_VCO_NOISE+AUDIO_VCO_LOW+AUDIO_ON

	defb AUDIO_ENVELOPE_PASSTHROUGH+AUDIO_MIXER_SILENCE+AUDIO_ON
	defb 0 ; end of instrument

; snare
song_instrument_1:
	defb AUDIO_ENVELOPE_VCO+AUDIO_MIXER_VCO_NOISE+AUDIO_VCO_LOW+AUDIO_ON
	defb AUDIO_ENVELOPE_VCO+AUDIO_MIXER_VCO_NOISE+AUDIO_VCO_HIGH+AUDIO_ON
	defb AUDIO_ENVELOPE_VCO+AUDIO_MIXER_VCO_NOISE+AUDIO_VCO_LOW+AUDIO_ON
	defb AUDIO_ENVELOPE_VCO+AUDIO_MIXER_VCO_NOISE+AUDIO_VCO_HIGH+AUDIO_ON
	defb AUDIO_ENVELOPE_VCO+AUDIO_MIXER_VCO_NOISE+AUDIO_VCO_LOW+AUDIO_ON
	defb AUDIO_ENVELOPE_VCO+AUDIO_MIXER_VCO_NOISE+AUDIO_VCO_HIGH+AUDIO_ON
	defb AUDIO_ENVELOPE_VCO+AUDIO_MIXER_VCO_NOISE+AUDIO_VCO_LOW+AUDIO_ON

	defb AUDIO_ENVELOPE_PASSTHROUGH+AUDIO_MIXER_SILENCE+AUDIO_ON
	defb 0 ; end of instrument

; alien beoep
song_instrument_2:
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_SLF_VCO+AUDIO_VCO_SLF2+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_SLF_VCO+AUDIO_VCO_SLF1+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_SLF_VCO+AUDIO_VCO_SLF2+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_SLF_VCO+AUDIO_VCO_SLF1+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_SLF_VCO+AUDIO_VCO_SLF2+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_SLF_VCO+AUDIO_VCO_SLF1+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_SLF_VCO+AUDIO_VCO_SLF2+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_SLF_VCO+AUDIO_VCO_SLF1+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_SLF_VCO+AUDIO_VCO_SLF2+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_SLF_VCO+AUDIO_VCO_SLF1+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_SLF_VCO+AUDIO_VCO_SLF2+AUDIO_ON


	defb AUDIO_ENVELOPE_PASSTHROUGH+AUDIO_MIXER_SILENCE+AUDIO_ON
	defb 0 ; end of instrument

; low pitch piip
song_instrument_3:
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_VCO+AUDIO_VCO_LOW+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_VCO+AUDIO_VCO_LOW+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_VCO+AUDIO_VCO_LOW+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_VCO+AUDIO_VCO_LOW+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_VCO+AUDIO_VCO_LOW+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_VCO+AUDIO_VCO_LOW+AUDIO_ON

	defb AUDIO_ENVELOPE_PASSTHROUGH+AUDIO_MIXER_SILENCE+AUDIO_ON
	defb 0 ; end of instrument


; high pitch piip
song_instrument_4:
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_VCO+AUDIO_VCO_HIGH+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_VCO+AUDIO_VCO_HIGH+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_VCO+AUDIO_VCO_HIGH+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_VCO+AUDIO_VCO_HIGH+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_VCO+AUDIO_VCO_HIGH+AUDIO_ON
	defb AUDIO_ENVELOPE_MONOVIPPA+AUDIO_MIXER_VCO+AUDIO_VCO_HIGH+AUDIO_ON

	defb AUDIO_ENVELOPE_PASSTHROUGH+AUDIO_MIXER_SILENCE+AUDIO_ON
	defb 0 ; end of instrument
