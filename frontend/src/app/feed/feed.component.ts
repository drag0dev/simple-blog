import { HttpClientModule } from '@angular/common/http';
import { Component } from '@angular/core';
import { BlogpostService } from '../services/blogpost.service';
import { Blogpost } from '../models/blogpost.model';
import { FeedBlogpostComponent } from './feed-blogpost/feed-blogpost.component';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-feed',
  standalone: true,
  imports: [HttpClientModule, FeedBlogpostComponent, CommonModule],
  providers: [BlogpostService],
  templateUrl: './feed.component.html',
  styleUrl: './feed.component.scss'
})
export class FeedComponent {
  constructor(private blogpostService: BlogpostService) {
    this.blogpostService = blogpostService;
  }

  public blogposts: Blogpost[] = [];
  public page: Number = 1;

  ngOnInit(): void {
    this.fetchFeed();
  }

  fetchFeed() {

    let resp = this.blogpostService.getFeed(this.page);
    resp.subscribe(
      (data: string) => {
        let dataParsed = JSON.parse(data);
        this.blogposts = dataParsed.blogposts;
      },
      _ => {
        alert('Error loading feed!')
      }
    )
  }
}
