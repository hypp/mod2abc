

	GLOBAL tracker_start
	GLOBAL tracker_init
	GLOBAL tracker_play

tracker_start:
	call tracker_init
	call wait_vbl

.loop:
	call wait_vbl
	call tracker_play

;	jr .loop
	jp .loop

wait_vbl:
	ld hl,$fdf0
	ld a,(hl)
.wait:
	cp (hl)
	jp z,.wait
	ret

tracker_init:
	ld a,(song_speed)
	ld (tracker_row_countdown),a

	ld a,TRACKER_PATTERN_SIZE
	ld (tracker_pattern_countdown),a

	ld hl,song_pattern_list
	ld (tracker_pattern_list),hl

	ld e,(hl)
	inc hl
	ld d,(hl)
	ex de,hl
	ld (tracker_current_pattern),hl

	ld a,(hl)
	cp -1
	jp z,.no_new_instrument

	ld hl,song_instrument_list
	ld b,0
	add a
	rl b
	ld c,a
	add hl,bc
	ld e,(hl)
	inc hl
	ld d,(hl)
	ex de,hl
	ld (tracker_current_instrument),hl
.no_new_instrument:

	ret

tracker_play:
	ld a,(tracker_row_countdown)
	dec a
	jp p,tracker_no_new_row
tracker_new_row:
	ld a,(song_speed)
	ld (tracker_row_countdown),a

	ld a,(tracker_pattern_countdown)
	dec a
	jp p,tracker_no_new_pattern
tracker_new_pattern:
	ld a,TRACKER_PATTERN_SIZE
	ld (tracker_pattern_countdown),a

	ld hl,(tracker_pattern_list)
	inc hl
	inc hl

	ld e,(hl)
	inc hl
	ld d,(hl)
	dec hl

	ld a,e
	or d
	jp nz,.no_restart

	ld hl,song_pattern_list
	ld e,(hl)
	inc hl
	ld d,(hl)
	dec hl

.no_restart:
	ld (tracker_pattern_list),hl

	ex de,hl
	ld (tracker_current_pattern),hl

	jp tracker_do_row

tracker_no_new_pattern:
	ld (tracker_pattern_countdown),a
	ld hl,(tracker_current_pattern)
	inc hl
	ld (tracker_current_pattern),hl

; hl must contain (tracker_current_pattern) 	
tracker_do_row:
	ld a,(hl)
	cp -1
	jp z,.no_new_instrument

	ld hl,song_instrument_list
	ld b,0
	add a
	rl b
	ld c,a
	add hl,bc
	ld e,(hl)
	inc hl
	ld d,(hl)
	ex de,hl
	ld (tracker_current_instrument),hl

.no_new_instrument:
	jp tracker_play_instrument

tracker_no_new_row:	
	ld (tracker_row_countdown),a

tracker_play_instrument:
	; Just play the current instrument
	; get pointer to current instrument
	ld hl,(tracker_current_instrument)

	; get current position within the instrument
	ld a,(hl)
	cp 0
	jp z,.instrument_done
	out (AUDIO_PORT),a

	inc hl
	ld (tracker_current_instrument),hl

.instrument_done:
	ret

; variables needed during playback
tracker_current_instrument:
	defw tracker_empty_instument
tracker_row_countdown:
	defb 0
tracker_pattern_list:
	defw 0
tracker_current_pattern:
	defw 0
tracker_pattern_countdown:
	defb 0

tracker_empty_instument:
	defb AUDIO_ENVELOPE_PASSTHROUGH+AUDIO_MIXER_SILENCE+AUDIO_ON
	defb 0 ; end of instrument

; Size of each pattern-1
TRACKER_PATTERN_SIZE=64-1

AUDIO_PORT=6

AUDIO_ENVELOPE_VCO = %00000000
AUDIO_ENVELOPE_PASSTHROUGH = %01000000
AUDIO_ENVELOPE_MONOVIPPA = %10000000
AUDIO_ENVELOPE_VCO_ALT_POL = %11000000

AUDIO_MIXER_VCO=%00000000
AUDIO_MIXER_NOISE=%00001000
AUDIO_MIXER_SLF=%00010000
AUDIO_MIXER_VCO_NOISE=%00011000
AUDIO_MIXER_SLF_NOISE=%00100000
AUDIO_MIXER_SLF_VCO=%00101000
AUDIO_MIXER_SLF_VCO_NOISE=%00110000
AUDIO_MIXER_SILENCE=%00111000

AUDIO_VCO_HIGH=%00000000
AUDIO_VCO_LOW=%00000010
AUDIO_VCO_SLF1=%00000100
AUDIO_VCO_SLF2=%00000110

AUDIO_ON=%00000001
AUDIO_OFF=%00000000

; nr of frames to next row in pattern-1
song_speed: 
	defb 6-1

	include 'instruments.s'
	include 'song.s'

;	SECTION aligned

;    SECTION .bss,"uR"


