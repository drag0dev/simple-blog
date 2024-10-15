import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CreateBlogPostDTO } from '../models/create-blogpost-dto.model';
import { BlogpostService } from '../services/blogpost.service';
import { HttpClientModule } from '@angular/common/http';

@Component({
  selector: 'app-new-post',
  standalone: true,
  imports: [FormsModule, CommonModule, HttpClientModule],
  providers: [BlogpostService],
  templateUrl: './new-post.component.html',
  styleUrl: './new-post.component.scss'
})
export class NewPostComponent {
  public errorMessage = "";
  public showErrorMessage = false;

  public username = "";
  public avatarURL = "";
  public text = "";
  public postImageFile: File | null = null;
  public postImage: Blob | null = null;
  public isPostDisabled = false;

  constructor(private blogpostService: BlogpostService) {
    this.blogpostService = blogpostService;
  }

  onFileSelected(files: FileList | null) {
     if (files && files.length > 0) {
       this.postImageFile = files[0];
       this.loadPostImage();
    }
  }

  onSubmit() {
    this.errorMessage = "";
    this.showErrorMessage = false;
    this.isPostDisabled = true;

    if (this.username.length == 0) {
      this.errorMessage = "Username is required!";
      this.showErrorMessage = true;
      this.isPostDisabled = false;
      return;
    } else if (this.username.length > 128) {
      this.errorMessage = "Username cannot be longer than 128 characters!";
      this.showErrorMessage = true;
      this.isPostDisabled = false;
      return;
    }

    if (this.text.length == 0) {
      this.errorMessage = "Text of the post is required!";
      this.showErrorMessage = true;
      this.isPostDisabled = false;
      return;
    } else if (this.text.length > 2000) {
      this.errorMessage = "Text of the post cannot be longer than 2000 characters!";
      this.showErrorMessage = true;
      this.isPostDisabled = false;
      return;
    }

    if (this.postImageFile != null && this.postImage == null) {
      this.isPostDisabled = false;
      return;
    }

    let dto: CreateBlogPostDTO = {
      text: this.text,
      username: this.username,
      avatar: this.avatarURL.length != 0 ? this.avatarURL : null
    }

    let resp = this.blogpostService.create(dto, this.postImage);
    resp.subscribe(
      _ => {
        location.reload();
      },
      err => {
        if (err.status == 400) {
          try {
            this.errorMessage = err.error['error'];
          } catch {
            this.errorMessage = "Unable to create a post, try again later!";
          }
        } else {
          this.errorMessage = "Unable to create a post, try again later!";
        }
        this.showErrorMessage = true;
        this.isPostDisabled = false;
      }
    )

  }

  loadPostImage() {
    if (this.postImageFile == null) return;
    const reader = new FileReader();

    reader.onload = () => {
      const byteArray = new Uint8Array(reader.result as ArrayBuffer);
      const pngSignature = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
      const isPng = pngSignature.every((byte, index) => byteArray[index] === byte);

      if (!isPng) {
        this.errorMessage = 'Post image is not a PNG!';
        this.showErrorMessage = true;
        this.postImage = null;
      } else if (byteArray.length > 2*1024*1024)  {
        this.errorMessage = 'Post image cannot be larger than 2MB!';
        this.showErrorMessage = true;
        this.postImage = null;
      } else this.postImage = new Blob([reader.result as ArrayBuffer], {type: 'application/octet-stream'});
    }

    reader.onerror = (_) => {
      this.errorMessage = 'Unable to load post image!';
      this.showErrorMessage = true;
      this.postImage = null;
    }

    reader.readAsArrayBuffer(this.postImageFile);
  }
}
