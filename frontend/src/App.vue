<script setup>
import { ref } from 'vue'

const photos = ref([])
const watermark = ref(null)
const processing = ref(false)
const message = ref('')

const fotoInput = ref(null)
const watermarkInput = ref(null)

const handlePhotos = (files) => {
  photos.value = [...files]
}

const handleWatermark = (files) => {
  watermark.value = files[0]
}

const openFotoPicker = () => {
  fotoInput.value.click()
}

const openWatermarkPicker = () => {
  watermarkInput.value.click()
}

const startProcess = async () => {
  if (photos.value.length === 0 || !watermark.value) {
    message.value = 'Harap unggah foto & watermark!'
    return
  }

  processing.value = true
  message.value = ''

  const form = new FormData()
  photos.value.forEach((file) => form.append('photos', file))
  form.append('watermark', watermark.value)

  try {
    const res = await fetch('http://127.0.0.1:3000/api/watermark', {
      method: 'POST',
      body: form,
    })

    const blob = await res.blob()
    const url = URL.createObjectURL(blob)

    const a = document.createElement('a')
    a.href = url
    a.download = 'hasil_watermark.zip'
    a.click()

    message.value = 'Berhasil! ZIP berhasil didownload.'
  } catch (err) {
    message.value = 'Gagal memproses foto.'
  }

  processing.value = false
}
</script>

<template>
  <div class="wrapper">
    <div class="card">
      <h1 class="title">Aplikasi Watermark Paralel</h1>

      <!-- Foto -->
      <div
        class="dropzone"
        @click="openFotoPicker"
        @dragover.prevent
        @drop.prevent="handlePhotos($event.dataTransfer.files)"
      >
        <p>Tarik banyak foto ke sini atau klik</p>
        <p class="count">{{ photos.length }} foto dipilih</p>
        <input
          type="file"
          ref="fotoInput"
          multiple
          hidden
          @change="handlePhotos($event.target.files)"
        />
      </div>

      <!-- Watermark -->
      <div
        class="dropzone"
        @click="openWatermarkPicker"
        @dragover.prevent
        @drop.prevent="handleWatermark($event.dataTransfer.files)"
      >
        <p>Tarik Watermark PNG ke sini</p>
        <p class="count">{{ watermark ? watermark.name : 'Belum dipilih' }}</p>
        <input
          type="file"
          ref="watermarkInput"
          accept="image/png"
          hidden
          @change="handleWatermark($event.target.files)"
        />
      </div>

      <button class="btn" :disabled="processing" @click="startProcess">
        {{ processing ? 'Memproses...' : 'Proses Foto' }}
      </button>

      <p class="message">{{ message }}</p>
    </div>
  </div>
</template>

<style>
.wrapper {
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  background: #0d0d0d;
}

.card {
  background: #1a1a1a;
  width: 420px;
  padding: 28px;
  border-radius: 14px;
  text-align: center;
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.6);
}

.title {
  color: white;
  margin-bottom: 24px;
}

.dropzone {
  border: 2px dashed #555;
  padding: 20px;
  margin-bottom: 20px;
  border-radius: 10px;
  cursor: pointer;
  color: #aaa;
}

.count {
  font-size: 12px;
  color: #888;
}

.btn {
  background: #4da3ff;
  width: 100%;
  padding: 12px;
  border-radius: 8px;
  border: none;
  color: white;
  font-weight: bold;
  cursor: pointer;
}

.btn:disabled {
  background: #333;
  cursor: not-allowed;
}

.message {
  margin-top: 10px;
  color: #ff8080;
}
</style>
