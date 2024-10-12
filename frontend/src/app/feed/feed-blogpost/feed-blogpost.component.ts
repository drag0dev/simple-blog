import { CommonModule } from '@angular/common';
import { HttpClientModule } from '@angular/common/http';
import { Component, ElementRef, Input, ViewChild } from '@angular/core';
import { ImageService } from '../../services/image.service';

@Component({
  selector: 'app-feed-blogpost',
  standalone: true,
  imports: [HttpClientModule, CommonModule],
  providers: [ImageService],
  templateUrl: './feed-blogpost.component.html',
  styleUrl: './feed-blogpost.component.scss'
})
export class FeedBlogpostComponent {
  constructor(private imageService: ImageService) {
    this.imageService = imageService;
  }
  @ViewChild('blogpostText') blogPostText!: ElementRef
  @Input() text: String = ''
  @Input() username: String = ''
  @Input() dateOfPublication: String = ''
  @Input() avatarId: String | null = null
  @Input() postImageId: String | null = null
  public avatarImage: String | null = null;
  public postImage: String | null = null;

  ngOnInit(): void {
    this.getAvatar()
    this.getPostImage()
  }

  ngAfterViewInit() {
    if (this.postImage == null) {
      this.blogPostText.nativeElement.style.width = '90%';
    }
  }

  getAvatar() {
    let id = this.avatarId;
    if (id == null) id = 'placeholder_avatar';
    let resp = this.imageService.get(id);
    resp.subscribe(
      avatar => {
        const objectUrl = URL.createObjectURL(avatar);
        this.avatarImage = objectUrl;
      },
      err => {
        console.log(err)
      }
    )
  }

  getPostImage() {
    if (this.postImageId == null) return;
    let resp = this.imageService.get(this.postImageId);
    resp.subscribe(
      postImage => {
        const objectUrl = URL.createObjectURL(postImage);
        this.postImage = objectUrl;
      },
      err => {
        console.log(err)
      }
    )
  }
}
