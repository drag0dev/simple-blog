import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FeedBlogpostComponent } from './feed-blogpost.component';

describe('FeedBlogpostComponent', () => {
  let component: FeedBlogpostComponent;
  let fixture: ComponentFixture<FeedBlogpostComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FeedBlogpostComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(FeedBlogpostComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
